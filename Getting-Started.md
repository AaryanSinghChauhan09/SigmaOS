1


Welcome to the SigmaOS Sovereign Lattice. This guide will help you set up your development environment and run your first SigmaOS instance.


1


Before you begin, ensure you have the following installed:


1



1



1
<<<<<<< HEAD



1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


The SigmaOS build system is modular and shard-based.


1



1


make all

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


We provide a specialized boot script that handles ISO creation and QEMU orchestration.


1



1


./qemu-boot.sh

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1


To view the kernel logs in real-time:


1

<<<<<<< HEAD

tail -f serial.log


=======
tail -f serial.log

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


If you want to interact with the experimental web-based UI:


1

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
npm install
node server.js


1



1



1



1

<<<<<<< HEAD


1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

---


1

