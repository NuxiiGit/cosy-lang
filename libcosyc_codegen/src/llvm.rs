use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};
use libcosyc_ir::ir;
use inkwell::{ context::Context, builder::Builder, module::Module };

/// Manages generation of code from IR.
pub struct Codegen<'a, 'ctx> {
    src : &'a str,
    issues : &'a mut IssueTracker,
    context : &'ctx Context,
    module : Module<'ctx>,
    builder : Builder<'ctx>,
}

impl Failable for Codegen<'_, '_> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl Renderable for Codegen<'_, '_> {
    fn src(&self) -> &str {
        self.src
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    /// Creates a new code generator from this LLVM context.
    pub fn new(context : &'ctx Context, module_name : &str, src : &'a str, issues : &'a mut IssueTracker) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self { src, issues, context, module, builder }
    }

    /// Generates the `main` entrypoint.
    pub fn generate_main(&self) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(main_block);
    }

    /// Emits LLVM IR to stderr.
    pub fn print_ir_to_stderr(&self) {
        self.module.print_to_stderr();
    }
}

/// Compiles the LLVM-IR for this instruction.
pub fn compile_ir(inst : ir::Inst, src : &str, issues : &mut IssueTracker) -> Option<()> {
    let context = Context::create();
    let codegen = Codegen::new(&context, "mod", src, issues);
    codegen.generate_main();
    codegen.print_ir_to_stderr();
    Some(())
}
