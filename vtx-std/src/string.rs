use std::collections::VecDeque;

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

        static "split", "(Ljava/lang/String;)[Ljava/lang/String;" { // this is literally not static
                                                                    // LMFAOO i just need to write
                                                                    // the ability to actually
                                                                    // parse that..
            let value_ref = running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.get("value").unwrap().clone();
            let mut value = VecDeque::<Argument>::from(running_in.get_array(Into::<usize>::into(value_ref.clone())).1.clone());
            let delimiter_ref = running_in.get_instance(Into::<usize>::into(args.get(1).unwrap().clone())).fields.get("value").unwrap().clone();
            let delimiter = running_in.get_array(Into::<usize>::into(delimiter_ref.clone())).1.clone();
            let mut array_out: Vec<Argument> = vec![];
            let mut working_array: Vec<Argument>;
            for i in 0..(value.len() - (delimiter.len() - 1)) {
                let mut valueslice: Vec<Argument> = vec![];
                for j in 0..(delimiter.len()) {
                    valueslice.push(value.get(i + j).unwrap().clone());
                }
                if valueslice == delimiter {
                    working_array = value.drain(0..i).collect();
                    let stringarr = running_in.create_array(working_array.clone(), MethodType::Char);
                    let class_handle = running_in.load_or_get_class_handle("java/lang/String".to_string());
                    let mut class = running_in.get_class(class_handle);
                    let instance_ref = running_in.prepare_instance(&mut class);
                    running_in.get_instance(instance_ref.try_into().unwrap()).fields.insert("value".to_string(), Argument::new(stringarr, MethodType::ArrayReference));
                    value.drain(0..delimiter.len());
                    array_out.push(Argument::new(instance_ref, MethodType::ClassReference {classpath: "java/lang/String".to_string()}));
                }
            }
            if array_out.is_empty() {
                array_out.push(args.get(0).unwrap().clone());
            }
            let outarr = running_in.create_array(array_out.clone(), MethodType::ClassReference {classpath: "java/lang/String".to_string()});
            Argument::new(outarr, MethodType::ArrayReference)
        }

        static "equals", "(Ljava/lang/Object;)Z" {
            let value_ref = running_in.get_instance(Into::<usize>::into(args.get(0).unwrap().clone())).fields.get("value").unwrap().clone();
            let mut value = running_in.get_array(Into::<usize>::into(value_ref.clone())).1.clone();
            let other_ref = running_in.get_instance(Into::<usize>::into(args.get(1).unwrap().clone())).fields.get("value").unwrap().clone();
            let other = running_in.get_array(Into::<usize>::into(other_ref.clone())).1.clone();
            if value.len() != other.len() {
                return Argument::new(0, MethodType::Boolean);
            }
            return Argument::new((value == other) as u32, MethodType::Boolean);
        }
    }
);
