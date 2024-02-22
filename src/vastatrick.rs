use std::{sync::{Arc, mpsc::{Sender, Receiver, self}}, collections::HashMap, pin::Pin};

use crate::{class::{Class, descriptor::{MethodDescriptor, FieldDescriptor}}, VastatrixRequest, VastatrixResponse, VastatrixRequestKind};

//one singular unit of the collection of vastatrix.
pub struct Vastatrick {
    cache: HashMap<String, Arc<dyn Class>>, //is this necessary?
    requester: VastatrickRequester,
}

impl Vastatrick {
    pub fn new(requester: Sender<VastatrixRequest>) -> Self {
	Self {
	    requester: VastatrickRequester::new(requester),
	    cache: HashMap::new(),
	}
    }

    pub fn run_main_class(&mut self, classpath: String) {
	let main_class = self.requester.resolve_class(classpath);
	let descriptor = MethodDescriptor {
	    parameters: vec![FieldDescriptor::ArrayReference(Box::new(FieldDescriptor::ClassReference(String::from("java.lang.String"))))],
	    returns: None,
	};
	main_class.call_method(String::from("main"), descriptor, &self.requester);
    }
}

pub struct VastatrickRequester {
    requester: Sender<VastatrixRequest>,
    response_sender: Arc<Sender<VastatrixResponse>>,
    responses: Receiver<VastatrixResponse>,
}

impl VastatrickRequester {
    pub fn new(requester: Sender<VastatrixRequest>) -> Self {
	let (response_sender, responses) = mpsc::channel();
	Self {
	    requester,
	    response_sender: Arc::new(response_sender),
	    responses,
	}	
    }

    pub fn exit(&self, code: i32) -> ! {
	self.send_request(VastatrixRequestKind::Exit(code));
	loop {}
    }

    pub fn resolve_class(&self, classpath: String) -> Arc<dyn Class> {
	let response = self.send_request(VastatrixRequestKind::ResolveClass(classpath));
	if let VastatrixResponse::ResolvedClass(Some(class)) = response {
	    class
	} else {
	    panic!("Wrong response! was: {:?}", response);
	}
    }

    fn send_request(&self, request: VastatrixRequestKind) -> VastatrixResponse {
	self.requester.send(VastatrixRequest {
	    kind: request,
	    responder: Arc::clone(&self.response_sender),
	}).unwrap();
	self.responses.recv().unwrap()
    }
}
