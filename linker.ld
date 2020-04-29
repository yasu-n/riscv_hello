OUTPUT_ARCH(riscv)
ENTRY(_start)


SECTIONS
{
	. = 0x80000000;
	.reset : {
		KEEP(*(.boot));
	}

	.text : {
		*(.text. text.*);
	}

	.rodata : {
		*(.rdata .rodata. rodata.*);
	}

	.data : {
		*(.data .data.*);
	}

	_bss_start = .;
	.bss : {
		*(.bss .bss.*);
	}
}
