#import <Foundation/Foundation.h>
#include <stddef.h>
#include <string.h>
#include <unistd.h>
#include <stdio.h>


typedef void(^GetMediaInfoCompletion)(CFDictionaryRef info);
void register_playstate_change_callback(void (*callback)(void)){
    dispatch_queue_t q = dispatch_queue_create("get_playing_change_info", DISPATCH_QUEUE_SERIAL);


    CFBundleRef bundle = CFBundleCreate(kCFAllocatorDefault, CFURLCreateWithFileSystemPath(kCFAllocatorSystemDefault,
        CFSTR("/System/Library/PrivateFrameworks/MediaRemote.framework"), kCFURLPOSIXPathStyle, true));
    void * register_func = CFBundleGetFunctionPointerForName(bundle, CFSTR("MRMediaRemoteRegisterForNowPlayingNotifications"));

    const void (*MRMediaRemoteRegisterForNowPlayingNotifications)(dispatch_queue_t) = register_func;

    void * info_func = CFBundleGetFunctionPointerForName(bundle, CFSTR("MRMediaRemoteGetNowPlayingInfo"));
    const void (*MRMediaRemoteGetNowPlayingInfo)(dispatch_queue_t, GetMediaInfoCompletion) = info_func;
    __block NSString *lastIdentifier = @"";
    [NSNotificationCenter.defaultCenter addObserverForName:@"kMRMediaRemoteNowPlayingInfoDidChangeNotification" object:nil queue:nil usingBlock:^(NSNotification * _Nonnull note) {
        MRMediaRemoteGetNowPlayingInfo(q, ^(CFDictionaryRef info) {
            NSString* id = (NSString *) CFDictionaryGetValue(info, CFSTR("kMRMediaRemoteNowPlayingInfoTitle"));
            if (id && ![id isEqualToString:(lastIdentifier)]) {
                NSLog(@"%@ %@", id, lastIdentifier);
                callback();
                lastIdentifier = [id copy];
            }
        });
    }];
    MRMediaRemoteRegisterForNowPlayingNotifications(q);

}

