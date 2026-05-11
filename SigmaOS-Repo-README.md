1


> Tiny queue data structure

You should use this package instead of an array if you do a lot of `Array#push()` and `Array#shift()` on large arrays, since `Array#shift()` has [linear time complexity](https://medium.com/@ariel.salem1989/an-easy-to-use-guide-to-big-o-time-complexity-5dcf4be8a444#:~:text=O(N)%E2%80%94Linear%20Time) *O(n)* while `Queue#dequeue()` has [constant time complexity](https://medium.com/@ariel.salem1989/an-easy-to-use-guide-to-big-o-time-complexity-5dcf4be8a444#:~:text=O(1)%20%E2%80%94%20Constant%20Time) *O(1)*. That makes a huge difference for large arrays.

> A [queue](https://en.wikipedia.org/wiki/Queue_(abstract_data_type)) is an ordered list of elements where an element is inserted at the end of the queue and is removed from the front of the queue. A queue works based on the first-in, first-out ([FIFO](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics))) principle.


1



1


npm install yocto-queue


1



1



1


const Queue = require('yocto-queue');

const queue = new Queue();

queue.enqueue('??');
queue.enqueue('??');

console.log(queue.size);
//=> 2

console.log(...queue);
//=> '?? ??'

console.log(queue.dequeue());
//=> '??'

console.log(queue.dequeue());
//=> '??'


1



1



1


The instance is an [`Iterable`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols), which means you can iterate over the queue front to back with a “for…of” loop, or use spreading to convert the queue to an array. Don't do this unless you really need to though, since it's slow.


1


Add a value to the queue.


1


Remove the next value in the queue.

Returns the removed value or `undefined` if the queue is empty.


1


Clear the queue.


1


The size of the queue.


1



1

