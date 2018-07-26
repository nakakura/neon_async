#[macro_use]
extern crate neon;

use std::thread;

use neon::vm::{Call, JsResult};
use neon::js::{JsBoolean, JsUndefined, JsNumber, JsFunction};
use neon::scope::Scope;
use neon::task::Task;

struct BgTask{
    sec: u32
}

impl Task for BgTask {
    type Output = ();
    type Error = ();
    type JsEvent = JsNumber;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        println!("async_sleep {} ms", self.sec * 1000);
        thread::sleep_ms(self.sec * 1000);
        Ok(())
    }

    fn complete<'a, T: Scope<'a>>(
        self,
        scope: &'a mut T,
        _result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        println!("Main Background Task Complete");
        Ok(JsNumber::new(scope, 3.0f64))
    }
}

fn async_sleep(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let sleep_sec = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value() as u32;
    let callback = call.arguments.require(scope, 1)?.check::<JsFunction>()?;

    (BgTask{sec: sleep_sec}).schedule(callback);
    Ok(JsNumber::new(scope, 2.0f64))
}

fn sleep(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let sleep_sec: u32 = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value() as u32;
    thread::sleep_ms(sleep_sec * 1000);
    Ok(JsNumber::new(scope, sleep_sec as f64))
}

register_module!(m, {
    m.export("sleep", sleep)?;
    m.export("async_sleep", async_sleep)?;
    Ok(())
});
