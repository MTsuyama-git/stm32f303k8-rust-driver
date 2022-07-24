/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
FLASH : ORIGIN = 0x08000000, LENGTH = 64K
RAM   : ORIGIN = 0x20000000, LENGTH = 12K
}

ENTRY(Reset);
EXTERN(RESET_VECTOR);

SECTIONS
{
	.vector_table ORIGIN(FLASH) :
	{
		/* First entry: inital Stack Pointer value */
		LONG(ORIGIN(RAM) + LENGTH(RAM));
		
		/* KEEP(*(.vector_table.reset_vector)); */

          	/* Second Entry: vector table */
		KEEP(*(.vector_table.exceptions));
	} > FLASH

	.text :
	{
		*(.text .text.*);
	} > FLASH

	.rodata :
	{
		*(.rodata .rodata.*);
	} > FLASH
	
	.bss (NOLOAD):
	{
		_sbss = .;
		*(.bss .bss.*);
		_ebss = .;
	} > RAM

	.data : AT(ADDR(.rodata) + SIZEOF(.rodata))
	{
		_sdata = .;
		*(.data .data.*);
		_edata = .;
	} > RAM

	_sidata = LOADADDR(.data);

	/DISCARD/ :
	{
		*(.ARM.exidx .ARM.exidx.*);
	}
}

PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);
PROVIDE(WWDG = DefaultExceptionHandler);
PROVIDE(PVD = DefaultExceptionHandler);
PROVIDE(TamperStamp = DefaultExceptionHandler);
PROVIDE(RTCWKUP = DefaultExceptionHandler);
PROVIDE(FLASH = DefaultExceptionHandler);
PROVIDE(RCC = DefaultExceptionHandler);
PROVIDE(EXTI0 = DefaultExceptionHandler);
PROVIDE(EXTI1 = DefaultExceptionHandler);
PROVIDE(EXTI2_TS = DefaultExceptionHandler);
PROVIDE(EXTI3 = DefaultExceptionHandler);
PROVIDE(EXTI4 = DefaultExceptionHandler);
PROVIDE(DMA1_C1 = DefaultExceptionHandler);
PROVIDE(DMA1_C2 = DefaultExceptionHandler);
PROVIDE(DMA1_C3 = DefaultExceptionHandler);
PROVIDE(DMA1_C4 = DefaultExceptionHandler);
PROVIDE(DMA1_C5 = DefaultExceptionHandler);
PROVIDE(DMA1_C6 = DefaultExceptionHandler);
PROVIDE(DMA1_C7 = DefaultExceptionHandler);
PROVIDE(ADC1_2 = DefaultExceptionHandler);
PROVIDE(CAN_TX = DefaultExceptionHandler);
PROVIDE(CAN_RX0 = DefaultExceptionHandler);
PROVIDE(CAN_RX1 = DefaultExceptionHandler);
PROVIDE(CAN_SCE = DefaultExceptionHandler);
PROVIDE(EXTI9_5 = DefaultExceptionHandler);
PROVIDE(TIM1_BRK = DefaultExceptionHandler);
PROVIDE(TIM1_UP = DefaultExceptionHandler);
PROVIDE(TIM1_TRG_COM = DefaultExceptionHandler);
PROVIDE(TIM1_CC = DefaultExceptionHandler);
PROVIDE(TIM2 = DefaultExceptionHandler);
PROVIDE(TIM3 = DefaultExceptionHandler);
PROVIDE(I2C1_EV = DefaultExceptionHandler);
PROVIDE(I2C1_ER = DefaultExceptionHandler);
PROVIDE(SPI1 = DefaultExceptionHandler);
PROVIDE(USART1 = DefaultExceptionHandler);
PROVIDE(USART2 = DefaultExceptionHandler);
PROVIDE(USART3 = DefaultExceptionHandler);
