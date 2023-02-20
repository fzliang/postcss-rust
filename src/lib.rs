mod postcss;
use std::time::Instant;

use neon::prelude::*;

fn parse(mut cx: FunctionContext) -> JsResult<JsString> {
    let css_argument = cx.argument::<JsString>(0)?;
    

    let css_string = css_argument.value(&mut cx);

    let start = Instant::now();
    postcss::parse(&css_string);
    let duration = start.elapsed();
    println!("postcss-rust rust: {:?}", duration);

    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    Ok(())
}
