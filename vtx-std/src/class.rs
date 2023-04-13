use vtx_jbridge::class;

class!(
    package java.lang;

    public class jClass {
        field instance "classpath", "Ljava/lang/String;";
        
        static "<init>", "()V" {
            Argument::new(0, MethodType::Void)
        }

        static "forName", "(Ljava/lang/String;)Ljava/lang/Class;" {
            let class_handle = running_in.load_or_get_class_handle("java/lang/Class".to_string());
            let mut class = running_in.get_class(class_handle);
            let instance = running_in.prepare_instance(&mut class);
            running_in.get_instance(instance.try_into().unwrap()).fields.insert("classpath".to_string(), args.get(0).unwrap().clone());
            Argument::new(instance, MethodType::ClassReference { classpath: "java/lang/Class".to_string() })
        }

        static "getMethod", "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;" {
            let classpath_ref = args.get(0).expect("no argument 1").clone();
            let meep = running_in.get_instance(Into::<usize>::into(classpath_ref)).fields.get("classpath").unwrap().clone();
            let methodname_ref = args.get(2).expect("no argument 2").clone();
            let methoddesc = args.get(1).expect("no argument 3").clone();
            let mut mdesc = Argument::new(0, MethodType::ClassReference { classpath: "java/lang/String".to_string() });
            let types = running_in.get_array(Into::<usize>::into(methoddesc.clone())).1.clone();
            for t in types {
                println!("MORP: {:?}", t);
                mdesc = running_in.get_instance(Into::<usize>::into(t)).fields.get("classpath").unwrap().clone();
            }
            let method_handle = running_in.load_or_get_class_handle("java/lang/reflect/Method".to_string()).clone();
            let mut class = running_in.get_class(method_handle).clone();
            let method_instance = running_in.prepare_instance(&mut class).clone();
            let instance = running_in.get_instance(method_instance.try_into().unwrap());
            instance.fields.insert("classpath".to_string(), meep.clone());
            instance.fields.insert("methodname".to_string(), methodname_ref);
            instance.fields.insert("methoddesc".to_string(), mdesc.clone());
            println!("{:?}", instance.fields);
            Argument::new(method_instance, MethodType::ClassReference { classpath: "java/lang/reflect/Method".to_string() })
        }
    }
);
