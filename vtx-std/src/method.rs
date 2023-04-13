use vtx_jbridge::class;

class!(
    package java.lang.reflect;

    public class Method {
        superclass java.lang.reflect.Executable;

        field instance "classpath", "Ljava/lang/String;";
        field instance "methodname", "Ljava/lang/String;";
        field instance "methoddesc", "Ljava/lang/String;";

        static "invoke", "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;" {
            let classpath_str_ref = running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.get("classpath").unwrap().clone();
            let classpath_arr_ref = running_in.get_instance(Into::<usize>::into(classpath_str_ref)).fields.get("value").unwrap().clone();
            let classpath_arr = running_in.get_array(Into::<usize>::into(classpath_arr_ref)).1.clone();
            let mut classpath_str = String::new();
            for char in classpath_arr {
                classpath_str.push(char::from_u32(Into::<usize>::into(char) as u32).unwrap());
            }
            classpath_str = classpath_str.replace(".", "/");
            println!("Classpath: {}", classpath_str);
            let methodname_str_ref = running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.get("methodname").unwrap().clone();
            let methodname_arr_ref = running_in.get_instance(Into::<usize>::into(methodname_str_ref)).fields.get("value").unwrap().clone();
            let methodname_arr = running_in.get_array(Into::<usize>::into(methodname_arr_ref)).1.clone();
            let mut methodname_str = String::new();
            for char in methodname_arr {
                methodname_str.push(char::from_u32(Into::<usize>::into(char) as u32).unwrap());
            }
            println!("Methodname: {}", methodname_str);
            let methoddesc_str_ref = running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.get("methoddesc").unwrap().clone();
            let methoddesc_arr_ref = running_in.get_instance(Into::<usize>::into(methoddesc_str_ref)).fields.get("value").unwrap().clone();
            let methoddesc_arr = running_in.get_array(Into::<usize>::into(methoddesc_arr_ref)).1.clone();
            let mut methoddesc_str = String::new();
            for char in methoddesc_arr {
                methoddesc_str.push(char::from_u32(Into::<usize>::into(char) as u32).unwrap());
            }
            methoddesc_str = format!("({})V", methoddesc_str);
            println!("Methoddesc: {}", methoddesc_str);
            
            let class = running_in.load_or_get_class_handle(classpath_str);
            let class = running_in.get_class(class);
            return class.create_frame(methodname_str, methoddesc_str).unwrap().exec(vec![], running_in);
        }
    }
);
