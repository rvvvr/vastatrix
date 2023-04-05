use vtx_jbridge::class;

class!(
    package java.lang;

    public class System {
        static "<init>", "()V" {
            Argument::new(0, MethodType::Void) 
        }

        static "getProperty", "(Ljava/lang/String;)Ljava/lang/String;" {
            let class_handle = running_in.load_or_get_class_handle("java/lang/String".to_string());
            let mut class = running_in.get_class(class_handle);
            let instance_ref = running_in.prepare_instance(&mut class); // i need better methods
                                                                        // for working with
                                                                        // strings... i'll write
                                                                        // them later.
            let mut string_as_arr: Vec<Argument> = vec![Argument::new('1' as u32, MethodType::Char), Argument::new('7' as u32, MethodType::Char)];
            let array = running_in.create_array(string_as_arr, MethodType::Char);
            let args = vec![Argument::new(instance_ref, MethodType::ClassReference {classpath: "java/lang/String".to_string()} ), Argument::new(array, MethodType::ArrayReference)];
            let constructor_frame = class.create_frame("<init>".to_string(), "([C)V".to_string()).unwrap().exec(args, running_in);
            return Argument::new(instance_ref, MethodType::ClassReference {classpath: "java/lang/String".to_string()});
        }
    }
);
