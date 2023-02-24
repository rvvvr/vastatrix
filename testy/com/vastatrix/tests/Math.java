package com.vastatrix.tests;

public class Math {
	public static int add(int a, int b) {
		return a + b;
	}

	public static int sub(int a, int b) {
		return a - b;
	}

	public static int mul(int a, int b) {
		return a * b;
	}

	public static int div(int a, int b) {
		return a / b;
	}

	public static int fib(int iters) {
		int a = 0;
		int b = 1;
		int c = 0;

		for(int i = 0; i < iters; i++) {
			c = add(a, b); //switching it up
			a = b;
			b = c;
		}
		return c;
	}

	public static float fadd(float a, float b, float c, float d, float e, float f) { //i also intended to test the load		index instructions with this but i got there without.
		return a + b + c + d + e + f; // will get to this later.
	}

	public static void varargs(int... a) {
		return;
	}

	public static int instantiate() {
		Test test = new Test();
		return test.zero();
	}

	public static class Test {
		private int zero;

		public Test() {
			zero = 0;
		}

		public int zero() {
			return zero;
		}
	}
}