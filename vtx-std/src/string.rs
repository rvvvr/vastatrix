use vtx_jbridge::class;

class!(
    package java.lang;

    public class jString {

        field instance "value", "[C";

        static "<init>", "()V" {
            let array = running_in.create_array(vec![], MethodType::Char);
            running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.insert("value".to_string(), Argument::new(array, MethodType::ArrayReference));
            Argument::new(0, MethodType::Void)
        }

        static "<init>", "([C)V" {
            let arrayref = args.get(1).unwrap(); 
            if !arrayref.is(MethodType::ArrayReference) {
                panic!("wrong type passed!") // type checking will be implicit later, for now we
                                             // check excplicitly inside the function.
            }
            let array = running_in.get_array(Into::<usize>::into(arrayref.clone())).1.clone();
            let newarray = running_in.create_array(array, MethodType::Char);
            running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.insert("value".to_string(), Argument::new(newarray, MethodType::ArrayReference));
            Argument::new(0, MethodType::Void)
        }

        static "split", "()V" {
            Argument::new(0, MethodType::Void)
        }
    }
);
