# Run Queue Policy Specification

Run queues own Ready tasks and provide bounded enqueue, selection, removal, and
wake operations. Queue state is protected for SMP access, and a task appears in
at most one runnable queue at a time.

A queue-full or allocation failure is explicit and observable. Tests cover FIFO
or priority ordering, duplicate admission, removal, migration, and starvation
signals before a policy is promoted.

