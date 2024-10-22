use super::*;
use parking_lot::{Condvar, Mutex};
use std::{sync::Arc, time::Duration};

impl CTPMarket {
    /// 从环境变量读取参数并初始化
    /// 不需要初始化回调
    pub fn env(self) -> CTPResult<Self> {
        self.init_by_env(empty!(_))?;
        Ok(self)
    }

    /// 从环境变量中读取参数并初始化
    /// 行情前置不需要权限, 但是需要完成登陆
    #[allow(clippy::mutex_atomic)]
    #[allow(clippy::missing_panics_doc)]
    pub fn init_by_env<F>(&self, mut cb: F) -> CTPResult<&Self>
    where
        F: Fn(Api) -> CTPApiResult + Send,
    {
        let env = &self.env;
        let api = self.api.clone();
        // 使用条件变量, 初始化的时候等待登陆完成
        let cdv = Wait::new();
        let mut cdv1 = cdv.clone();

        let login_field = CThostFtdcReqUserLoginField::new()
            .with_broker_id(&env.broker_id)
            .with_user_id(&env.user_id)
            .with_password(&env.user_password)
            .build();

        self.on_front_connected(move |api| {
            tracing::info!("行情前置服务器连接");
            let mut api = api.lock();
            api.req_user_login(&login_field, 0)
        })
        .on_rsp_user_login(move |_, user, error, _, _| {
            tracing::info!("行情前置服务器登陆");
            let api = api.clone();
            tracing::info!(FrontID = user.FrontID, "行情初始化");
            cdv1.wake(());
            cb(api)
        });

        self.register_front(&env.front).init();

        // 如果 5s 之内未连接, 不再等待
        let duration = Duration::from_secs(5);
        if cdv.timeout(duration).is_some() {
            // 这里的等待绝对不会超时
            self.ready.until(true);
            Ok(self)
        } else {
            Err(CTPError::TimeOut)
        }
    }
}
