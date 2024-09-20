#[cfg(feature = "stm32h747cm4")]
use super::Vector;

#[cfg(feature = "stm32h747cm4")]
extern "C" {
    fn WWDG2();
    fn PVD_PVM();
    fn RTC_TAMP_STAMP_CSS_LSE();
    fn RTC_WKUP();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STR0();
    fn DMA1_STR1();
    fn DMA1_STR2();
    fn DMA1_STR3();
    fn DMA1_STR4();
    fn DMA1_STR5();
    fn DMA1_STR6();
    fn ADC1_2();
    fn FDCAN1_IT0();
    fn FDCAN2_IT0();
    fn FDCAN1_IT1();
    fn FDCAN2_IT1();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STR7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STR0();
    fn DMA2_STR1();
    fn DMA2_STR2();
    fn DMA2_STR3();
    fn DMA2_STR4();
    fn ETH();
    fn ETH_WKUP();
    fn FDCAN_CAL();
    fn CM7_SEV_IT();
    fn DMA2_STR5();
    fn DMA2_STR6();
    fn DMA2_STR7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn DMA2D();
    fn SAI2();
    fn QUADSPI();
    fn LPTIM1();
    fn CEC();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPDIF();
    fn OTG_FS_EP1_OUT();
    fn OTG_FS_EP1_IN();
    fn OTG_FS_WKUP();
    fn OTG_FS();
    fn DMAMUX1_OV();
    fn HRTIM1_MST();
    fn HRTIM1_TIMA();
    fn HRTIM_TIMB();
    fn HRTIM1_TIMC();
    fn HRTIM1_TIMD();
    fn HRTIM_TIME();
    fn HRTIM1_FLT();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SAI3();
    fn SWPMI1();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn MDIOS_WKUP();
    fn MDIOS();
    fn JPEG();
    fn MDMA();
    fn DSI();
    fn SDMMC();
    fn HSEM0();
    fn ADC3();
    fn DMAMUX2_OVR();
    fn BDMA_CH1();
    fn BDMA_CH2();
    fn BDMA_CH3();
    fn BDMA_CH4();
    fn BDMA_CH5();
    fn BDMA_CH6();
    fn BDMA_CH7();
    fn BDMA_CH8();
    fn COMP();
    fn LPTIM2();
    fn LPTIM3();
    fn LPTIM4();
    fn LPTIM5();
    fn LPUART();
    fn WWDG1_RST();
    fn CRS();
    fn RAMECC();
    fn SAI4();
    fn HOLD_CORE();
    fn WKUP();
}

#[cfg(feature = "stm32h747cm4")]
#[link_section = ".hopter_vector_table_m4.interrupts_m4"]
#[no_mangle]
pub static __HOPTER_INTERRUPTS_M4: [Vector; 150] = [
    Vector { handler: WWDG2 },
    Vector { handler: PVD_PVM },
    Vector {
        handler: RTC_TAMP_STAMP_CSS_LSE,
    },
    Vector { handler: RTC_WKUP },
    Vector { reserved: 0 },
    Vector { handler: RCC },
    Vector { handler: EXTI0 },
    Vector { handler: EXTI1 },
    Vector { handler: EXTI2 },
    Vector { handler: EXTI3 },
    Vector { handler: EXTI4 },
    Vector { handler: DMA1_STR0 },
    Vector { handler: DMA1_STR1 },
    Vector { handler: DMA1_STR2 },
    Vector { handler: DMA1_STR3 },
    Vector { handler: DMA1_STR4 },
    Vector { handler: DMA1_STR5 },
    Vector { handler: DMA1_STR6 },
    Vector { handler: ADC1_2 },
    Vector {
        handler: FDCAN1_IT0,
    },
    Vector {
        handler: FDCAN2_IT0,
    },
    Vector {
        handler: FDCAN1_IT1,
    },
    Vector {
        handler: FDCAN2_IT1,
    },
    Vector { handler: EXTI9_5 },
    Vector { handler: TIM1_BRK },
    Vector { handler: TIM1_UP },
    Vector {
        handler: TIM1_TRG_COM,
    },
    Vector { handler: TIM_CC },
    Vector { handler: TIM2 },
    Vector { handler: TIM3 },
    Vector { handler: TIM4 },
    Vector { handler: I2C1_EV },
    Vector { handler: I2C1_ER },
    Vector { handler: I2C2_EV },
    Vector { handler: I2C2_ER },
    Vector { handler: SPI1 },
    Vector { handler: SPI2 },
    Vector { handler: USART1 },
    Vector { handler: USART2 },
    Vector { handler: USART3 },
    Vector { handler: EXTI15_10 },
    Vector { handler: RTC_ALARM },
    Vector { reserved: 0 },
    Vector {
        handler: TIM8_BRK_TIM12,
    },
    Vector {
        handler: TIM8_UP_TIM13,
    },
    Vector {
        handler: TIM8_TRG_COM_TIM14,
    },
    Vector { handler: TIM8_CC },
    Vector { handler: DMA1_STR7 },
    Vector { handler: FMC },
    Vector { handler: SDMMC1 },
    Vector { handler: TIM5 },
    Vector { handler: SPI3 },
    Vector { handler: UART4 },
    Vector { handler: UART5 },
    Vector { handler: TIM6_DAC },
    Vector { handler: TIM7 },
    Vector { handler: DMA2_STR0 },
    Vector { handler: DMA2_STR1 },
    Vector { handler: DMA2_STR2 },
    Vector { handler: DMA2_STR3 },
    Vector { handler: DMA2_STR4 },
    Vector { handler: ETH },
    Vector { handler: ETH_WKUP },
    Vector { handler: FDCAN_CAL },
    Vector {
        handler: CM7_SEV_IT,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: DMA2_STR5 },
    Vector { handler: DMA2_STR6 },
    Vector { handler: DMA2_STR7 },
    Vector { handler: USART6 },
    Vector { handler: I2C3_EV },
    Vector { handler: I2C3_ER },
    Vector {
        handler: OTG_HS_EP1_OUT,
    },
    Vector {
        handler: OTG_HS_EP1_IN,
    },
    Vector {
        handler: OTG_HS_WKUP,
    },
    Vector { handler: OTG_HS },
    Vector { handler: DCMI },
    Vector { handler: CRYP },
    Vector { handler: HASH_RNG },
    Vector { handler: FPU },
    Vector { handler: UART7 },
    Vector { handler: UART8 },
    Vector { handler: SPI4 },
    Vector { handler: SPI5 },
    Vector { handler: SPI6 },
    Vector { handler: SAI1 },
    Vector { handler: LTDC },
    Vector { handler: LTDC_ER },
    Vector { handler: DMA2D },
    Vector { handler: SAI2 },
    Vector { handler: QUADSPI },
    Vector { handler: LPTIM1 },
    Vector { handler: CEC },
    Vector { handler: I2C4_EV },
    Vector { handler: I2C4_ER },
    Vector { handler: SPDIF },
    Vector {
        handler: OTG_FS_EP1_OUT,
    },
    Vector {
        handler: OTG_FS_EP1_IN,
    },
    Vector {
        handler: OTG_FS_WKUP,
    },
    Vector { handler: OTG_FS },
    Vector {
        handler: DMAMUX1_OV,
    },
    Vector {
        handler: HRTIM1_MST,
    },
    Vector {
        handler: HRTIM1_TIMA,
    },
    Vector {
        handler: HRTIM_TIMB,
    },
    Vector {
        handler: HRTIM1_TIMC,
    },
    Vector {
        handler: HRTIM1_TIMD,
    },
    Vector {
        handler: HRTIM_TIME,
    },
    Vector {
        handler: HRTIM1_FLT,
    },
    Vector {
        handler: DFSDM1_FLT0,
    },
    Vector {
        handler: DFSDM1_FLT1,
    },
    Vector {
        handler: DFSDM1_FLT2,
    },
    Vector {
        handler: DFSDM1_FLT3,
    },
    Vector { handler: SAI3 },
    Vector { handler: SWPMI1 },
    Vector { handler: TIM15 },
    Vector { handler: TIM16 },
    Vector { handler: TIM17 },
    Vector {
        handler: MDIOS_WKUP,
    },
    Vector { handler: MDIOS },
    Vector { handler: JPEG },
    Vector { handler: MDMA },
    Vector { handler: DSI },
    Vector { handler: SDMMC },
    Vector { handler: HSEM0 },
    Vector { reserved: 0 },
    Vector { handler: ADC3 },
    Vector {
        handler: DMAMUX2_OVR,
    },
    Vector { handler: BDMA_CH1 },
    Vector { handler: BDMA_CH2 },
    Vector { handler: BDMA_CH3 },
    Vector { handler: BDMA_CH4 },
    Vector { handler: BDMA_CH5 },
    Vector { handler: BDMA_CH6 },
    Vector { handler: BDMA_CH7 },
    Vector { handler: BDMA_CH8 },
    Vector { handler: COMP },
    Vector { handler: LPTIM2 },
    Vector { handler: LPTIM3 },
    Vector { handler: LPTIM4 },
    Vector { handler: LPTIM5 },
    Vector { handler: LPUART },
    Vector { handler: WWDG1_RST },
    Vector { handler: CRS },
    Vector { handler: RAMECC },
    Vector { handler: SAI4 },
    Vector { reserved: 0 },
    Vector { handler: HOLD_CORE },
    Vector { handler: WKUP },
];
