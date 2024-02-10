package com.example.mysimpleparcelableservice;

import com.example.mysimpleparcelableservice.MySimpleParcelable;

interface IMySimpleParcelableService {
    /** Generate a MySimpleParcelable message. */
    MySimpleParcelable sendMySimpleParcelable(String name, int years);
}