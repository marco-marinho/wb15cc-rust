.syntax unified
.cpu cortex-m4
.thumb

.global g_pfnVectors
.global Default_Handler


/**
 * @brief  This is the code that gets called when the processor receives an
 *         unexpected interrupt.  This simply enters an infinite loop, preserving
 *         the system state for examination by a debugger.
 *
 * @param  None
 * @retval : None
*/
  .section .text.Default_Handler,"ax",%progbits
Default_Handler:
Infinite_Loop:
  b Infinite_Loop
  .size Default_Handler, .-Default_Handler

/**
*
* The STM32WB15CCUx vector table.  Note that the proper constructs
* must be placed on this to ensure that it ends up at physical address
* 0x0000.0000.
*
*/

  .section .isr_vector,"a",%progbits
  .type g_pfnVectors, %object
  .size g_pfnVectors, .-g_pfnVectors

g_pfnVectors:
  .word _estack
  .word Reset_Handler
  .word NMI_Handler
  .word HardFault_Handler
  .word	MemManage_Handler
  .word	BusFault_Handler
  .word	UsageFault_Handler
  .word	0
  .word	0
  .word	0
  .word	0
  .word	SVC_Handler
  .word	DebugMon_Handler
  .word	0
  .word	PendSV_Handler
  .word	SysTick_Handler
  .word	WWDG_IRQHandler         			/* Window Watchdog interrupt                                          */
  .word	PVD_IRQHandler          			/* PVD through EXTI[16] (C1IMR2[20])                                  */
  .word	RTC_TAMP_IRQHandler     			/* RTC/TAMP/CSS on LSE through EXTI line 19 interrupt                 */
  .word	RTC_WKUP_IRQHandler     			/* RTC wakeup interrupt through EXTI[19]                              */
  .word	FLASH_IRQHandler        			/* Flash global interrupt                                             */
  .word	RCC_IRQHandler          			/* RCC global interrupt                                               */
  .word	EXTI0_IRQHandler        			/* EXTI line 0 interrupt through EXTI[0]                              */
  .word	EXTI1_IRQHandler        			/* EXTI line 0 interrupt through EXTI[1]                              */
  .word	EXTI2_IRQHandler        			/* EXTI line 0 interrupt through EXTI[2]                              */
  .word	EXTI3_IRQHandler        			/* EXTI line 0 interrupt through EXTI[3]                              */
  .word	EXTI4_IRQHandler        			/* EXTI line 0 interrupt through EXTI[4]                              */
  .word	DMA1_CH1_IRQHandler     			/* DMA1 Channel1 global interrupt                                     */
  .word	DMA1_CH2_IRQHandler     			/* DMA1 Channel2 global interrupt                                     */
  .word	DMA1_CH3_IRQHandler     			/* DMA1 Channel3 interrupt                                            */
  .word	DMA1_CH4_IRQHandler     			/* DMA1 Channel4 interrupt                                            */
  .word	DMA1_CH5_IRQHandler     			/* DMA1 Channel5 interrupt                                            */
  .word	DMA1_CH6_IRQHandler     			/* DMA1 Channel6 interrupt                                            */
  .word	DMA1_CH7_IRQHandler     			/* DMA1 Channel 7 interrupt                                           */
  .word	ADC1_IRQHandler         			/* ADC1 global interrupt                                              */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	C2SEV_IRQHandler        			/* CPU2 SEV through EXTI[40]                                          */
  .word	COMP_IRQHandler         			/* COMP1 interrupt through EXTI[21:20]                                */
  .word	EXTI5_9_IRQHandler      			/* EXTI line [9:5] interrupt through EXTI[9:5]                        */
  .word	TIM1_BRK_IRQHandler     			/* Timer 1 break interrupt                                            */
  .word	TIM1_UP_IRQHandler      			/* Timer 1 Update                                                     */
  .word	TIM1_TRG_COM_IRQHandler 			/* TIM1 Trigger and Commutation interrupts and TIM17 global interrupt */
  .word	TIM1_CC_IRQHandler      			/* TIM1 Capture Compare interrupt                                     */
  .word	TIM2_IRQHandler         			/* TIM2 global interrupt                                              */
  .word	PKA_IRQHandler          			/* Private key accelerator interrupt                                  */
  .word	I2C1_EV_IRQHandler      			/* I2C1 event interrupt                                               */
  .word	I2C1_ER_IRQHandler      			/* I2C1 error interrupt                                               */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	SPI1_IRQHandler         			/* SPI 1 global interrupt                                             */
  .word	0                       			/* Reserved                                                           */
  .word	USART1_IRQHandler       			/* USART1 global interrupt                                            */
  .word	LPUART1_IRQHandler      			/* LPUART1 global interrupt                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	EXTI10_15_IRQHandler    			/* EXTI line [15:10] interrupt through EXTI[15:10]                    */
  .word	RTC_ALARM_IRQHandler    			/* RTC Alarms (A and B) interrupt through AIEC                        */
  .word	0                       			/* Reserved                                                           */
  .word	PWR_SOTF_IRQHandler     			/* PWR switching on the fly interrupt                                 */
  .word	IPCC_C1_RX_IT_IRQHandler			/* IPCC CPU1 RX occupied interrupt                                    */
  .word	IPCC_C1_TX_IT_IRQHandler			/* IPCC CPU1 TX free interrupt                                        */
  .word	HSEM_IRQHandler         			/* Semaphore interrupt 0 to CPU1                                      */
  .word	LPTIM1_IRQHandler       			/* LPtimer 1 global interrupt                                         */
  .word	LPTIM2_IRQHandler       			/* LPtimer 2 global interrupt                                         */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	AES2_IRQHandler         			/* AES2 global interrupt                                              */
  .word	True_RNG_IRQHandler     			/* True random number generator interrupt                             */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	0                       			/* Reserved                                                           */
  .word	DMAMUX1_OVR_IRQHandler  			/* DMAMUX1 overrun interrupt                                          */

/*******************************************************************************
*
* Provide weak aliases for each Exception handler to the Default_Handler.
* As they are weak aliases, any function with the same name will override
* this definition.
*
*******************************************************************************/

	.weak	NMI_Handler
	.thumb_set NMI_Handler,Default_Handler

	.weak	HardFault_Handler
	.thumb_set HardFault_Handler,Default_Handler

	.weak	MemManage_Handler
	.thumb_set MemManage_Handler,Default_Handler

	.weak	BusFault_Handler
	.thumb_set BusFault_Handler,Default_Handler

	.weak	UsageFault_Handler
	.thumb_set UsageFault_Handler,Default_Handler

	.weak	SVC_Handler
	.thumb_set SVC_Handler,Default_Handler

	.weak	DebugMon_Handler
	.thumb_set DebugMon_Handler,Default_Handler

	.weak	PendSV_Handler
	.thumb_set PendSV_Handler,Default_Handler

	.weak	SysTick_Handler
	.thumb_set SysTick_Handler,Default_Handler

	.weak	WWDG_IRQHandler
	.thumb_set WWDG_IRQHandler,Default_Handler

	.weak	PVD_IRQHandler
	.thumb_set PVD_IRQHandler,Default_Handler

	.weak	RTC_TAMP_IRQHandler
	.thumb_set RTC_TAMP_IRQHandler,Default_Handler

	.weak	RTC_WKUP_IRQHandler
	.thumb_set RTC_WKUP_IRQHandler,Default_Handler

	.weak	FLASH_IRQHandler
	.thumb_set FLASH_IRQHandler,Default_Handler

	.weak	RCC_IRQHandler
	.thumb_set RCC_IRQHandler,Default_Handler

	.weak	EXTI0_IRQHandler
	.thumb_set EXTI0_IRQHandler,Default_Handler

	.weak	EXTI1_IRQHandler
	.thumb_set EXTI1_IRQHandler,Default_Handler

	.weak	EXTI2_IRQHandler
	.thumb_set EXTI2_IRQHandler,Default_Handler

	.weak	EXTI3_IRQHandler
	.thumb_set EXTI3_IRQHandler,Default_Handler

	.weak	EXTI4_IRQHandler
	.thumb_set EXTI4_IRQHandler,Default_Handler

	.weak	DMA1_CH1_IRQHandler
	.thumb_set DMA1_CH1_IRQHandler,Default_Handler

	.weak	DMA1_CH2_IRQHandler
	.thumb_set DMA1_CH2_IRQHandler,Default_Handler

	.weak	DMA1_CH3_IRQHandler
	.thumb_set DMA1_CH3_IRQHandler,Default_Handler

	.weak	DMA1_CH4_IRQHandler
	.thumb_set DMA1_CH4_IRQHandler,Default_Handler

	.weak	DMA1_CH5_IRQHandler
	.thumb_set DMA1_CH5_IRQHandler,Default_Handler

	.weak	DMA1_CH6_IRQHandler
	.thumb_set DMA1_CH6_IRQHandler,Default_Handler

	.weak	DMA1_CH7_IRQHandler
	.thumb_set DMA1_CH7_IRQHandler,Default_Handler

	.weak	ADC1_IRQHandler
	.thumb_set ADC1_IRQHandler,Default_Handler

	.weak	C2SEV_IRQHandler
	.thumb_set C2SEV_IRQHandler,Default_Handler

	.weak	COMP_IRQHandler
	.thumb_set COMP_IRQHandler,Default_Handler

	.weak	EXTI5_9_IRQHandler
	.thumb_set EXTI5_9_IRQHandler,Default_Handler

	.weak	TIM1_BRK_IRQHandler
	.thumb_set TIM1_BRK_IRQHandler,Default_Handler

	.weak	TIM1_UP_IRQHandler
	.thumb_set TIM1_UP_IRQHandler,Default_Handler

	.weak	TIM1_TRG_COM_IRQHandler
	.thumb_set TIM1_TRG_COM_IRQHandler,Default_Handler

	.weak	TIM1_CC_IRQHandler
	.thumb_set TIM1_CC_IRQHandler,Default_Handler

	.weak	TIM2_IRQHandler
	.thumb_set TIM2_IRQHandler,Default_Handler

	.weak	PKA_IRQHandler
	.thumb_set PKA_IRQHandler,Default_Handler

	.weak	I2C1_EV_IRQHandler
	.thumb_set I2C1_EV_IRQHandler,Default_Handler

	.weak	I2C1_ER_IRQHandler
	.thumb_set I2C1_ER_IRQHandler,Default_Handler

	.weak	SPI1_IRQHandler
	.thumb_set SPI1_IRQHandler,Default_Handler

	.weak	USART1_IRQHandler
	.thumb_set USART1_IRQHandler,Default_Handler

	.weak	LPUART1_IRQHandler
	.thumb_set LPUART1_IRQHandler,Default_Handler

	.weak	EXTI10_15_IRQHandler
	.thumb_set EXTI10_15_IRQHandler,Default_Handler

	.weak	RTC_ALARM_IRQHandler
	.thumb_set RTC_ALARM_IRQHandler,Default_Handler

	.weak	PWR_SOTF_IRQHandler
	.thumb_set PWR_SOTF_IRQHandler,Default_Handler

	.weak	IPCC_C1_RX_IT_IRQHandler
	.thumb_set IPCC_C1_RX_IT_IRQHandler,Default_Handler

	.weak	IPCC_C1_TX_IT_IRQHandler
	.thumb_set IPCC_C1_TX_IT_IRQHandler,Default_Handler

	.weak	HSEM_IRQHandler
	.thumb_set HSEM_IRQHandler,Default_Handler

	.weak	LPTIM1_IRQHandler
	.thumb_set LPTIM1_IRQHandler,Default_Handler

	.weak	LPTIM2_IRQHandler
	.thumb_set LPTIM2_IRQHandler,Default_Handler

	.weak	AES2_IRQHandler
	.thumb_set AES2_IRQHandler,Default_Handler

	.weak	True_RNG_IRQHandler
	.thumb_set True_RNG_IRQHandler,Default_Handler

	.weak	DMAMUX1_OVR_IRQHandler
	.thumb_set DMAMUX1_OVR_IRQHandler,Default_Handler

	.weak	SystemInit

/************************ (C) COPYRIGHT STMicroelectonics *****END OF FILE****/
