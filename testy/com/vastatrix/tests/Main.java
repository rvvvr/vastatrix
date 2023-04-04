package com.vastatrix.tests;

import com.vastatrix.tests.Math;
import com.vastatrix.tests.MoreMath;

class Main {
    public static void main(String[] args){
        int a = Math.add(Math.instantiate(), 5);
	int[] b = {5, 4, 3, 2, 1};
	int c = 0;
	for (int i = 0; i < b.length; i++) {
		c += b[i];
	}
    }
}
