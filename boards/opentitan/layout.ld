MEMORY
{
  rom   (rx)  : ORIGIN = 0x20000000, LENGTH = 0x30000
  prog  (rx)  : ORIGIN = 0x20030000, LENGTH = 0x100000-0x30000
  vpp   (rxw) : ORIGIN = 0x20033000, LENGTH = 0x100000-0x30000
  ram   (!rx) : ORIGIN = 0x10000000, LENGTH = 0x10000
}

MPU_MIN_ALIGN = 1K;
SECTIONS {
    /*
     * The flash header needs to match what the boot ROM for OpenTitan is
     * expecting.  At the moment, it contains only the entry point, but it
     * will eventually contain the signature -- and (hopefully?!) some
     * versioning information to make it slightly easier to debug when the
     * boot ROM and Tock are out of sync with respect to the definition...
     */
    .flash_header : {
        LONG(_stext)
    } > rom

    .vpp_app :  {
       . = ALIGN(4);
       _svppapp = .;

            /* Include a placeholder byte in this section so that the linker
             * includes a segment for it. Otherwise the section will be empty and
             * the linker will ignore it when defining the segments.
             */
            BYTE(0)
        } > vpp
        _evppapp = _svppapp + LENGTH(vpp);
}

INCLUDE ../kernel_layout.ld
