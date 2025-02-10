#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <pwd.h>
#include <sys/sysctl.h>
#include <mach/task.h>
#include <mach/vm_map.h>
#include <mach/thread_state.h>

#define ARRAY_SIZE(array) \
    (sizeof(array) / sizeof(*array))

#define CHECK_MACH(_msg, machret) do {                                      \
    if (machret != KERN_SUCCESS)                                        \
    {                                                                   \
        char _szError[1024];                                            \
        snprintf(_szError, ARRAY_SIZE(_szError), "%s: %u: %s", __FUNCTION__, __LINE__, _msg);  \
        mach_error(_szError, machret);                                  \
        abort();                                                        \
    }                                                                   \
} while (false)

void WEAK_MEMORY_BEGONE() {
    mach_msg_type_number_t cThreads;
    thread_act_t *pThreads;
    kern_return_t machret = task_threads(mach_task_self(), &pThreads, &cThreads);
    CHECK_MACH("task_threads()", machret);
    //printf("Number of threads: %u\n", cThreads);
    //exit(1);

    uintptr_t sp;
    uintptr_t registerValues[128];

    for (mach_msg_type_number_t i = 0; i < cThreads; i++)
    {
        size_t registers = 128;
        machret = thread_get_register_pointer_values(pThreads[i], &sp, &registers, registerValues);
        if (machret == KERN_INSUFFICIENT_BUFFER_SIZE)
        {
            CHECK_MACH("thread_get_register_pointer_values()", machret);
        }

        machret = mach_port_deallocate(mach_task_self(), pThreads[i]);
        CHECK_MACH("mach_port_deallocate()", machret);
    }

    machret = vm_deallocate(mach_task_self(), (vm_address_t)pThreads, cThreads * sizeof(thread_act_t));
    CHECK_MACH("vm_deallocate()", machret);
}
