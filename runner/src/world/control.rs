use std::ops::Range;
use gml::symbol::Symbol;
use gml::{self, vm};

#[derive(Default)]
pub struct State {
    next_id: i32,
}

#[gml::bind]
impl State {
    #[gml::api]
    pub fn execute_string(
        cx: &mut crate::Context, thread: &mut vm::Thread,
        str: Symbol, args: Range<usize>
    ) -> vm::Result<vm::Value> {
        let crate::Context { world, assets, .. } = cx;
        let crate::World { show, control, .. } = world;
        let crate::Assets { code: assets, .. } = assets;

        let id = control.next_id;
        control.next_id += 1;

        let errors = &mut *show.write;
        let prototypes = &assets.prototypes;

        let function = gml::Function::String { id };
        let name = gml::FunctionDisplay::String;
        let (code, locations, errors) = gml::compile_program(prototypes, name, &str[..], errors);
        if errors > 0 { return Ok(vm::Value::from(0.0)); }

        assets.code.insert(function, code);
        show.debug.locations.insert(function, locations);

        let args = Vec::from(unsafe { thread.arguments(args) });
        let result = thread.execute(cx, function, args);

        let crate::Context { world, assets, .. } = cx;
        let crate::World { show, .. } = world;
        let crate::Assets { code: assets, .. } = assets;

        assets.code.remove(&function);
        show.debug.locations.remove(&function);

        result
    }

    #[gml::api]
    pub fn script_execute(
        cx: &mut crate::Context, thread: &mut vm::Thread,
        scr: i32, args: Range<usize>
    ) -> vm::Result<vm::Value> {
        let scr = gml::Function::Script { id: scr };
        let args = Vec::from(unsafe { thread.arguments(args) });
        thread.execute(cx, scr, args)
    }

    #[gml::api]
    pub fn action_execute_script(
        cx: &mut crate::Context, thread: &mut vm::Thread, scr: i32, args: Range<usize>
    ) -> vm::Result<vm::Value> {
        Self::script_execute(cx, thread, scr, args)
    }
}
