use neon::prelude::*;

macro_rules! get_args {
    ($cx:expr, $i:expr) => {
        $cx.argument::<JsString>($i)?.value($cx)
    };
}

macro_rules! Okie {
    ($cx:expr) => {
        Ok($cx.undefined())
    };
}

fn make(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let src = get_args!(&mut cx, 0);
    let target = get_args!(&mut cx, 1);
    let strategy = get_args!(&mut cx, 2);

    Okie!(&mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("make", make)?;
    Ok(())
}
