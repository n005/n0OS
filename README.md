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
