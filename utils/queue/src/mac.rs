#[macro_export]
macro_rules! catch {
    ($env:ident, $stm:expr) => {
        match $stm {
            Ok(result) => result,
            Err(e) => {
                log::error!("{}", e);
                $env.throw_error(format!("{}", e))?;
                return $env.undefined();
            }
        }
    };

    ($env:ident, $stm:expr, $return:expr) => {
        match $stm {
            Ok(result) => result,
            Err(e) => {
                log::error!("{}", e);
                $env.throw_error(format!("{}", e))?;
                return $return;
            }
        }
    };
}
