static mut CALL_STACK: Vec<FunctionCall> = vec![];
static mut STACK_DEPTH: usize = 0;
pub const CALL_STACK_ENABLED: bool = false;
pub const MAX_ARG_LEN: usize = 128;

#[derive(Clone)]
struct FunctionCall {
    stack_depth: usize,
    args: String,
    name: String
}

pub fn push_call_stack(name: &str, args: &str) {

    if !CALL_STACK_ENABLED {
        return;
    }

    let name = name.to_string();
    let mut args = args.to_string();

    if args.len() > MAX_ARG_LEN || args.contains("\n") {
        args = String::from("...");
    }

    unsafe {
        CALL_STACK.push(
            FunctionCall {
                name,
                args,
                stack_depth: STACK_DEPTH
            }
        );
        STACK_DEPTH += 1;
    }
}

pub fn pop_call_stack() {

    if !CALL_STACK_ENABLED {
        return;
    }

    unsafe { STACK_DEPTH -= 1; }
}

pub fn print_call_stack() {
    unsafe {

        for func in CALL_STACK.iter() {
            println!("{}{}({})", " ".repeat(func.stack_depth * 4), func.name, func.args);
        }

    }
}