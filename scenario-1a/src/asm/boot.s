/*************************************************************************
 * place your boot code here
 *************************************************************************/
.global __boot
.global __rust_entry__

 .section .text.boot /* ensure this entrypoint links always to the start address */
__boot:
    // all the boot code should go here




    // after the boot code is done branch into the rust environment and do not return
    b __rust_entry__
    // as a fall back - if returned - hang
    b .hang

.hang:
    b .hang