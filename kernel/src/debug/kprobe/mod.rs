use crate::debug::kprobe::args::KprobeInfo;
use crate::libs::spinlock::SpinLock;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use kprobe::{Kprobe, KprobeBuilder, KprobeManager, KprobeOps, KprobePoint};
use system_error::SystemError;

mod args;
mod test;

pub static KPROBE_MANAGER: SpinLock<KprobeManager> = SpinLock::new(KprobeManager::new());
static KPROBE_POINT_LIST: SpinLock<BTreeMap<usize, Arc<KprobePoint>>> =
    SpinLock::new(BTreeMap::new());

pub fn kprobe_init() {}

#[cfg(feature = "kprobe_test")]
pub fn kprobe_test() {
    test::kprobe_test();
}

/// # 注册一个kprobe
///
/// 该函数会根据`symbol`查找对应的函数地址，如果找不到则返回错误。
///
/// ## 参数
/// - `kprobe_info`: kprobe的信息
pub fn register_kprobe(kprobe_info: KprobeInfo) -> Result<Arc<Kprobe>, SystemError> {
    let kprobe_builder = KprobeBuilder::try_from(kprobe_info)?;
    let address = kprobe_builder.probe_addr();
    let existed_point = KPROBE_POINT_LIST.lock().get(&address).map(Clone::clone);
    let kprobe = match existed_point {
        Some(existed_point) => {
            kprobe_builder
                .with_probe_point(existed_point.clone())
                .install()
                .0
        }
        None => {
            let (kprobe, probe_point) = kprobe_builder.install();
            KPROBE_POINT_LIST.lock().insert(address, probe_point);
            kprobe
        }
    };
    let kprobe = Arc::new(kprobe);
    KPROBE_MANAGER.lock().insert_kprobe(kprobe.clone());
    Ok(kprobe)
}

/// # 注销一个kprobe
///
/// ## 参数
/// - `kprobe`: 已安装的kprobe
pub fn unregister_kprobe(kprobe: Arc<Kprobe>) -> Result<(), SystemError> {
    let kprobe_addr = kprobe.probe_point().break_address();
    KPROBE_MANAGER.lock().remove_kprobe(&kprobe);
    // 如果没有其他kprobe注册在这个地址上，则删除探测点
    if KPROBE_MANAGER.lock().kprobe_num(kprobe_addr) == 0 {
        KPROBE_POINT_LIST.lock().remove(&kprobe_addr);
    }
    Ok(())
}