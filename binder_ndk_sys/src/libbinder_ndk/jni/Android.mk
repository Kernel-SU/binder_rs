LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE := binder_ndk
LOCAL_SRC_FILES := stub.c

include $(BUILD_SHARED_LIBRARY)