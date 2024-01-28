# n0OS
My Operating System

# How to

Generate floppy image (bootloader 512 bytes image):

``` nasm bootloader.asm -f bin -o boot.bin ```

Test it with qemu floppy:

``` qemu-system-x86_64 -fda boot.bin ```

Create floppy image:

``` dd if=/dev/zero of=floppy.img ibs=1k count=1440 ```

Copy image to floppy:

``` dd if=boot.bin of=floppy.img conv=notrunc ```

Create ISO image for CD/DVD:

``` mkisofs -b boot.bin -v -r -l -o bootable.iso cd ```

Test it with qemu CD/DVD:

``` qemu-system-x86_64 -boot d -cdrom bootable.iso ```

# Goals

- [x] Bootloader
- [x] Switch to Protected mode
- [x] Assembly to C/Rust transition
- [ ] Interrupt handling & CPU exceptions handling
- [ ] Double fault handling
- [ ] Tiny i/o (screen VGA text mode & keyboard) support
- [ ] Tiny libc/std library
- [ ] Memory management (paging, heap mem, virtual mem, dynamic memory allocation etc.)
- [ ] Filesystem
- [ ] Shell
- [ ] User space

# References

- [OSDev](https://wiki.osdev.org/Expanded_Main_Page)
- [os-tutorial](https://github.com/cfenollosa/os-tutorial)
- [Writing a Simple Operating System — from Scratch](https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Little OS book](https://littleosbook.github.io/)
- [Assembly docs](https://www.felixcloutier.com/x86/)
- [Gladir docs](https://www.gladir.com/CODER/ASM8086/index.htm)
- [Intermezzos](https://intermezzos.github.io)