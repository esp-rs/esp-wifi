use crate::{
    compat::{
        self,
        queue::SimpleQueue,
        timer_compat::{Timer, TIMERS},
    },
    memory_fence::memory_fence,
    preempt::preempt::task_create,
    timer::{get_systimer_count, yield_task},
};

pub fn init_tasks() {
    task_create(worker_task1);
    task_create(worker_task2);

    // if coex then we know we have ble + wifi
    #[cfg(coex)]
    task_create(worker_task3);
}

pub extern "C" fn worker_task1() {
    loop {
        compat::work_queue::do_work();
        yield_task();
    }
}

pub extern "C" fn worker_task3() {
    loop {
        compat::work_queue::do_work();
        yield_task();
    }
}

pub extern "C" fn worker_task2() {
    loop {
        let mut to_run: SimpleQueue<
            (
                fn(*mut crate::binary::c_types::c_void),
                *mut crate::binary::c_types::c_void,
            ),
            10,
        > = SimpleQueue::new();

        critical_section::with(|_| unsafe {
            let current_timestamp = get_systimer_count();
            memory_fence();
            for i in 0..TIMERS.len() {
                if let Some(old) = TIMERS[i] {
                    memory_fence();
                    if old.active && current_timestamp >= old.expire {
                        debug!("timer is due.... {:?}", old.ptimer);
                        let fnctn: fn(*mut crate::binary::c_types::c_void) =
                            core::mem::transmute(old.timer_ptr);
                        unwrap!(to_run.enqueue((fnctn, old.arg_ptr)));

                        TIMERS[i] = Some(Timer {
                            expire: if old.period != 0 {
                                current_timestamp + old.period
                            } else {
                                0
                            },
                            active: if old.period != 0 { old.active } else { false },
                            ..old
                        });
                    }
                };
            }
        });

        // run the due timer callbacks NOT in an interrupt free context
        while let Some((fnc, arg)) = to_run.dequeue() {
            trace!("trigger timer....");
            fnc(arg);
            trace!("timer callback called");
        }

        yield_task();
    }
}
