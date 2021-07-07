use anyhow::Result;
use cranelift::{
    codegen::{
        binemit::{NullStackMapSink, NullTrapSink},
        settings,
    },
    frontend::{FunctionBuilder, FunctionBuilderContext},
    prelude::{isa, types, AbiParam, InstBuilder},
};
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub fn codegen(name: &str, input: i64) -> Result<Vec<u8>> {
    let isa = isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));

    let mut module = ObjectModule::new(ObjectBuilder::new(
        isa,
        "low-level-lang",
        default_libcall_names(),
    )?);

    let mut context = module.make_context();

    context
        .func
        .signature
        .returns
        .push(AbiParam::new(types::I64));

    let mut builder_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

    let block = builder.create_block();
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);
    builder.seal_block(block);

    let number = builder.ins().iconst(types::I64, input);
    builder.ins().return_(&[number]);
    builder.finalize();

    let func = module.declare_function(name, Linkage::Export, &context.func.signature)?;

    module.define_function(
        func,
        &mut context,
        &mut NullTrapSink {},
        &mut NullStackMapSink {},
    )?;

    let object = module.finish().emit()?;

    Ok(object)
}
