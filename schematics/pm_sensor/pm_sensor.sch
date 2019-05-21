EESchema Schematic File Version 4
LIBS:pm_sensor-cache
EELAYER 29 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L MCU_Module:WeMos_D1_mini U1
U 1 1 5CDC60A2
P 2250 2650
F 0 "U1" H 2250 1650 50  0000 C CNN
F 1 "WeMos_D1_mini" H 2250 1550 50  0000 C CNN
F 2 "Module:WEMOS_D1_mini_light" H 2250 1500 50  0001 C CNN
F 3 "https://wiki.wemos.cc/products:d1:d1_mini#documentation" H 400 1500 50  0001 C CNN
	1    2250 2650
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x05_Male J1
U 1 1 5CDC8D2B
P 4150 2200
F 0 "J1" H 4258 2581 50  0000 C CNN
F 1 "sds011_conn" H 4258 2490 50  0000 C CNN
F 2 "Connector_JST:JST_XH_B5B-XH-A_1x05_P2.50mm_Vertical" H 4150 2200 50  0001 C CNN
F 3 "~" H 4150 2200 50  0001 C CNN
	1    4150 2200
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0101
U 1 1 5CDCAEEC
P 5000 2200
F 0 "#PWR0101" H 5000 1950 50  0001 C CNN
F 1 "GND" H 5005 2027 50  0000 C CNN
F 2 "" H 5000 2200 50  0001 C CNN
F 3 "" H 5000 2200 50  0001 C CNN
	1    5000 2200
	1    0    0    -1  
$EndComp
Wire Wire Line
	4350 2200 5000 2200
Text GLabel 3000 2750 2    50   Input ~ 0
sds_rx
Text GLabel 3000 2850 2    50   Output ~ 0
sds_tx
Wire Wire Line
	2650 2750 3000 2750
Wire Wire Line
	2650 2850 3000 2850
$Comp
L power:GND #PWR0104
U 1 1 5CDD1529
P 2850 1200
F 0 "#PWR0104" H 2850 950 50  0001 C CNN
F 1 "GND" H 2855 1027 50  0000 C CNN
F 2 "" H 2850 1200 50  0001 C CNN
F 3 "" H 2850 1200 50  0001 C CNN
	1    2850 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	2850 1200 3000 1200
Wire Wire Line
	3300 1500 3300 1800
Text GLabel 3000 2350 2    50   BiDi ~ 0
sda_3v
Text GLabel 3050 2450 2    50   Output ~ 0
scl_3v
Wire Wire Line
	2650 2350 3000 2350
Wire Wire Line
	2650 2450 3050 2450
Wire Wire Line
	4350 2000 4650 2000
Text GLabel 4650 2000 2    50   Input ~ 0
sds_rx
Wire Wire Line
	4350 2100 4650 2100
Text GLabel 4650 2100 2    50   Output ~ 0
sds_tx
Text GLabel 4050 3400 0    50   BiDi ~ 0
sda_3v
$Comp
L Transistor_FET:2N7000 Q1
U 1 1 5CDD4658
P 4450 3300
F 0 "Q1" V 4701 3300 50  0000 C CNN
F 1 "2N7000TA" V 4792 3300 50  0000 C CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline_Wide" H 4650 3225 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/BS/BSS138.pdf" H 4450 3300 50  0001 L CNN
	1    4450 3300
	0    1    1    0   
$EndComp
$Comp
L power:+3.3V #PWR0105
U 1 1 5CDDB7EF
P 4150 3000
F 0 "#PWR0105" H 4150 2850 50  0001 C CNN
F 1 "+3.3V" H 4165 3173 50  0000 C CNN
F 2 "" H 4150 3000 50  0001 C CNN
F 3 "" H 4150 3000 50  0001 C CNN
	1    4150 3000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R2
U 1 1 5CDDC466
P 4150 3200
F 0 "R2" H 4220 3246 50  0000 L CNN
F 1 "10k" H 4220 3155 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 4080 3200 50  0001 C CNN
F 3 "~" H 4150 3200 50  0001 C CNN
	1    4150 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	4050 3400 4150 3400
Wire Wire Line
	4150 3350 4150 3400
Connection ~ 4150 3400
Wire Wire Line
	4150 3400 4250 3400
Wire Wire Line
	4150 3000 4150 3050
$Comp
L Device:R R4
U 1 1 5CDDD43B
P 4750 3200
F 0 "R4" H 4820 3246 50  0000 L CNN
F 1 "10k" H 4820 3155 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 4680 3200 50  0001 C CNN
F 3 "~" H 4750 3200 50  0001 C CNN
	1    4750 3200
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0106
U 1 1 5CDDFA1E
P 4750 3000
F 0 "#PWR0106" H 4750 2850 50  0001 C CNN
F 1 "+5V" H 4765 3173 50  0000 C CNN
F 2 "" H 4750 3000 50  0001 C CNN
F 3 "" H 4750 3000 50  0001 C CNN
	1    4750 3000
	1    0    0    -1  
$EndComp
Wire Wire Line
	4750 3000 4750 3050
Wire Wire Line
	4650 3400 4750 3400
Wire Wire Line
	4750 3400 4750 3350
Wire Wire Line
	4750 3400 4950 3400
Connection ~ 4750 3400
Text GLabel 4950 3400 2    50   BiDi ~ 0
sda_5v
Text GLabel 4050 4350 0    50   Output ~ 0
scl_3v
$Comp
L power:+3.3V #PWR0107
U 1 1 5CDFE055
P 4150 3950
F 0 "#PWR0107" H 4150 3800 50  0001 C CNN
F 1 "+3.3V" H 4165 4123 50  0000 C CNN
F 2 "" H 4150 3950 50  0001 C CNN
F 3 "" H 4150 3950 50  0001 C CNN
	1    4150 3950
	1    0    0    -1  
$EndComp
$Comp
L Device:R R3
U 1 1 5CDFE05B
P 4150 4150
F 0 "R3" H 4220 4196 50  0000 L CNN
F 1 "10k" H 4220 4105 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 4080 4150 50  0001 C CNN
F 3 "~" H 4150 4150 50  0001 C CNN
	1    4150 4150
	1    0    0    -1  
$EndComp
Wire Wire Line
	4050 4350 4150 4350
Wire Wire Line
	4150 4300 4150 4350
Connection ~ 4150 4350
Wire Wire Line
	4150 4350 4250 4350
Wire Wire Line
	4150 3950 4150 4000
$Comp
L Device:R R5
U 1 1 5CDFE066
P 4750 4150
F 0 "R5" H 4820 4196 50  0000 L CNN
F 1 "10k" H 4820 4105 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 4680 4150 50  0001 C CNN
F 3 "~" H 4750 4150 50  0001 C CNN
	1    4750 4150
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0108
U 1 1 5CDFE06C
P 4750 3950
F 0 "#PWR0108" H 4750 3800 50  0001 C CNN
F 1 "+5V" H 4765 4123 50  0000 C CNN
F 2 "" H 4750 3950 50  0001 C CNN
F 3 "" H 4750 3950 50  0001 C CNN
	1    4750 3950
	1    0    0    -1  
$EndComp
Wire Wire Line
	4750 3950 4750 4000
Wire Wire Line
	4650 4350 4750 4350
Wire Wire Line
	4750 4350 4750 4300
Wire Wire Line
	4750 4350 4950 4350
Connection ~ 4750 4350
Text GLabel 4950 4350 2    50   Output ~ 0
scl_5v
Text GLabel 5950 900  0    50   BiDi ~ 0
sda_3v
$Comp
L Device:R R1
U 1 1 5CDFEBA7
P 3450 1800
F 0 "R1" V 3243 1800 50  0000 C CNN
F 1 "10k" V 3334 1800 50  0000 C CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 3380 1800 50  0001 C CNN
F 3 "~" H 3450 1800 50  0001 C CNN
	1    3450 1800
	0    1    1    0   
$EndComp
$Comp
L power:+3.3V #PWR0109
U 1 1 5CE00375
P 3600 1800
F 0 "#PWR0109" H 3600 1650 50  0001 C CNN
F 1 "+3.3V" V 3615 1928 50  0000 L CNN
F 2 "" H 3600 1800 50  0001 C CNN
F 3 "" H 3600 1800 50  0001 C CNN
	1    3600 1800
	0    1    1    0   
$EndComp
Text GLabel 5950 1000 0    50   Output ~ 0
scl_3v
Wire Wire Line
	5950 900  6400 900 
Wire Wire Line
	5950 1000 6400 1000
$Comp
L power:GND #PWR0110
U 1 1 5CE05E66
P 5950 1100
F 0 "#PWR0110" H 5950 850 50  0001 C CNN
F 1 "GND" H 5955 927 50  0000 C CNN
F 2 "" H 5950 1100 50  0001 C CNN
F 3 "" H 5950 1100 50  0001 C CNN
	1    5950 1100
	1    0    0    -1  
$EndComp
Wire Wire Line
	5950 1100 6400 1100
$Comp
L power:+3.3V #PWR0111
U 1 1 5CE07077
P 6400 1200
F 0 "#PWR0111" H 6400 1050 50  0001 C CNN
F 1 "+3.3V" V 6415 1328 50  0000 L CNN
F 2 "" H 6400 1200 50  0001 C CNN
F 3 "" H 6400 1200 50  0001 C CNN
	1    6400 1200
	0    -1   -1   0   
$EndComp
$Comp
L Connector:Conn_01x04_Male J3
U 1 1 5CE08ABF
P 6600 2000
F 0 "J3" H 6572 1882 50  0000 R CNN
F 1 "conn_display" H 6572 1973 50  0000 R CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 6600 2000 50  0001 C CNN
F 3 "~" H 6600 2000 50  0001 C CNN
	1    6600 2000
	-1   0    0    1   
$EndComp
Text GLabel 5950 1800 0    50   Output ~ 0
scl_5v
Text GLabel 5950 1900 0    50   BiDi ~ 0
sda_5v
$Comp
L power:GND #PWR0112
U 1 1 5CE0C0FA
P 5950 2100
F 0 "#PWR0112" H 5950 1850 50  0001 C CNN
F 1 "GND" H 5955 1927 50  0000 C CNN
F 2 "" H 5950 2100 50  0001 C CNN
F 3 "" H 5950 2100 50  0001 C CNN
	1    5950 2100
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0113
U 1 1 5CE0C9E0
P 5950 2000
F 0 "#PWR0113" H 5950 1850 50  0001 C CNN
F 1 "+5V" V 5965 2128 50  0000 L CNN
F 2 "" H 5950 2000 50  0001 C CNN
F 3 "" H 5950 2000 50  0001 C CNN
	1    5950 2000
	0    -1   -1   0   
$EndComp
Wire Wire Line
	5950 2000 6400 2000
Wire Wire Line
	5950 2100 6400 2100
Wire Wire Line
	5950 1900 6400 1900
Wire Wire Line
	5950 1800 6400 1800
$Comp
L power:+5V #PWR0114
U 1 1 5CE0F7E8
P 4200 900
F 0 "#PWR0114" H 4200 750 50  0001 C CNN
F 1 "+5V" H 4215 1073 50  0000 C CNN
F 2 "" H 4200 900 50  0001 C CNN
F 3 "" H 4200 900 50  0001 C CNN
	1    4200 900 
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0115
U 1 1 5CE106B0
P 4200 1200
F 0 "#PWR0115" H 4200 950 50  0001 C CNN
F 1 "GND" H 4205 1027 50  0000 C CNN
F 2 "" H 4200 1200 50  0001 C CNN
F 3 "" H 4200 1200 50  0001 C CNN
	1    4200 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C1
U 1 1 5CE11046
P 4200 1050
F 0 "C1" H 4315 1096 50  0000 L CNN
F 1 "100nF" H 4315 1005 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P5.00mm" H 4238 900 50  0001 C CNN
F 3 "~" H 4200 1050 50  0001 C CNN
	1    4200 1050
	1    0    0    -1  
$EndComp
$Comp
L Transistor_FET:2N7000 Q2
U 1 1 5CE152C1
P 4450 4250
F 0 "Q2" V 4701 4250 50  0000 C CNN
F 1 "2N7000TA" V 4792 4250 50  0000 C CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline_Wide" H 4650 4175 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/BS/BSS138.pdf" H 4450 4250 50  0001 L CNN
	1    4450 4250
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR0116
U 1 1 5CDD7496
P 2250 3450
F 0 "#PWR0116" H 2250 3200 50  0001 C CNN
F 1 "GND" H 2350 3450 50  0000 C CNN
F 2 "" H 2250 3450 50  0001 C CNN
F 3 "" H 2250 3450 50  0001 C CNN
	1    2250 3450
	1    0    0    -1  
$EndComp
Wire Wire Line
	4150 3000 4450 3000
Wire Wire Line
	4450 3000 4450 3100
Connection ~ 4150 3000
Wire Wire Line
	4150 3950 4450 3950
Wire Wire Line
	4450 3950 4450 4050
Connection ~ 4150 3950
$Comp
L New_Library:BMP180_breakout D1
U 1 1 5CDDA03B
P 6700 1050
F 0 "D1" H 6928 1101 50  0000 L CNN
F 1 "BMP180_breakout" H 6928 1010 50  0000 L CNN
F 2 "pm_sensor:BMP180_Breakout" H 6700 750 50  0001 C CNN
F 3 "" H 6700 750 50  0001 C CNN
	1    6700 1050
	1    0    0    -1  
$EndComp
NoConn ~ 1850 2250
NoConn ~ 1850 2550
NoConn ~ 1850 2650
NoConn ~ 2650 3050
NoConn ~ 2650 2550
NoConn ~ 2650 2150
$Comp
L power:+3.3V #PWR0117
U 1 1 5CDE0ACD
P 2350 1850
F 0 "#PWR0117" H 2350 1700 50  0001 C CNN
F 1 "+3.3V" H 2365 2023 50  0000 C CNN
F 2 "" H 2350 1850 50  0001 C CNN
F 3 "" H 2350 1850 50  0001 C CNN
	1    2350 1850
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0118
U 1 1 5CDE1726
P 2150 1850
F 0 "#PWR0118" H 2150 1700 50  0001 C CNN
F 1 "+5V" H 2165 2023 50  0000 C CNN
F 2 "" H 2150 1850 50  0001 C CNN
F 3 "" H 2150 1850 50  0001 C CNN
	1    2150 1850
	1    0    0    -1  
$EndComp
NoConn ~ 4350 2400
$Comp
L Transistor_FET:IRF9540N Q4
U 1 1 5CDE8BA4
P 9250 1450
F 0 "Q4" H 9456 1404 50  0000 L CNN
F 1 "IRF9510" H 9456 1495 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-220-3_Vertical" H 9450 1375 50  0001 L CIN
F 3 "http://www.irf.com/product-info/datasheets/data/irf9540n.pdf" H 9250 1450 50  0001 L CNN
F 4 "X" H 9250 1450 50  0001 C CNN "Spice_Primitive"
F 5 "irf9510" H 9250 1450 50  0001 C CNN "Spice_Model"
F 6 "Y" H 9250 1450 50  0001 C CNN "Spice_Netlist_Enabled"
F 7 "/Users/kolen/items/pm_sensor/schematics/sihf9510.lib" H 9250 1450 50  0001 C CNN "Spice_Lib_File"
F 8 "2 1 3" H 9250 1450 50  0001 C CNN "Spice_Node_Sequence"
	1    9250 1450
	1    0    0    1   
$EndComp
$Comp
L Device:R R7
U 1 1 5CDECCD9
P 8850 1200
F 0 "R7" H 8920 1246 50  0000 L CNN
F 1 "10k" H 8920 1155 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 8780 1200 50  0001 C CNN
F 3 "~" H 8850 1200 50  0001 C CNN
	1    8850 1200
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0102
U 1 1 5CDF1575
P 8850 2700
F 0 "#PWR0102" H 8850 2450 50  0001 C CNN
F 1 "GND" H 8855 2527 50  0000 C CNN
F 2 "" H 8850 2700 50  0001 C CNN
F 3 "" H 8850 2700 50  0001 C CNN
F 4 "I" H 8850 2700 50  0001 C CNN "Spice_Primitive"
F 5 "dc 0" H 8850 2700 50  0001 C CNN "Spice_Model"
F 6 "Y" H 8850 2700 50  0001 C CNN "Spice_Netlist_Enabled"
	1    8850 2700
	1    0    0    -1  
$EndComp
Wire Wire Line
	8850 1450 9050 1450
Wire Wire Line
	9350 950  9350 1250
Wire Wire Line
	8850 1350 8850 1450
Wire Wire Line
	8850 1050 8850 950 
Wire Wire Line
	8850 950  9350 950 
$Comp
L Device:R R8
U 1 1 5CE19895
P 8850 1700
F 0 "R8" H 8920 1746 50  0000 L CNN
F 1 "300" H 8920 1655 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 8780 1700 50  0001 C CNN
F 3 "~" H 8850 1700 50  0001 C CNN
	1    8850 1700
	1    0    0    -1  
$EndComp
$Comp
L Transistor_FET:2N7000 Q3
U 1 1 5CE19D61
P 8750 2050
F 0 "Q3" H 8956 2096 50  0000 L CNN
F 1 "2N7000TA" H 8956 2005 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline_Wide" H 8950 1975 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/2N/2N7000.pdf" H 8750 2050 50  0001 L CNN
F 4 "X" H 8750 2050 50  0001 C CNN "Spice_Primitive"
F 5 "2n7000" H 8750 2050 50  0001 C CNN "Spice_Model"
F 6 "Y" H 8750 2050 50  0001 C CNN "Spice_Netlist_Enabled"
F 7 "3 2 1" H 8750 2050 50  0001 C CNN "Spice_Node_Sequence"
F 8 "/Users/kolen/items/pm_sensor/schematics/2n7000.lib" H 8750 2050 50  0001 C CNN "Spice_Lib_File"
	1    8750 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	8850 1550 8850 1450
Connection ~ 8850 1450
$Comp
L Device:R R6
U 1 1 5CE2396C
P 8100 2050
F 0 "R6" V 7893 2050 50  0000 C CNN
F 1 "300" V 7984 2050 50  0000 C CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 8030 2050 50  0001 C CNN
F 3 "~" H 8100 2050 50  0001 C CNN
	1    8100 2050
	0    1    1    0   
$EndComp
$Comp
L power:+5V #PWR0119
U 1 1 5CE3400A
P 8650 950
F 0 "#PWR0119" H 8650 800 50  0001 C CNN
F 1 "+5V" V 8665 1078 50  0000 L CNN
F 2 "" H 8650 950 50  0001 C CNN
F 3 "" H 8650 950 50  0001 C CNN
	1    8650 950 
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8650 950  8850 950 
Connection ~ 8850 950 
Text GLabel 7850 2050 0    50   Output ~ 0
sds_switch
Text GLabel 9350 1650 3    50   Output ~ 0
sds_vcc
Text GLabel 4550 2300 2    50   Output ~ 0
sds_vcc
Wire Wire Line
	7850 2050 7950 2050
Text GLabel 2900 2250 2    50   Output ~ 0
sds_switch
NoConn ~ 2650 2650
$Comp
L Connector_Generic:Conn_01x03 J2
U 1 1 5CE58BB4
P 2400 1050
F 0 "J2" H 2480 1092 50  0000 L CNN
F 1 "Conn_01x03" H 2480 1001 50  0000 L CNN
F 2 "" H 2400 1050 50  0001 C CNN
F 3 "~" H 2400 1050 50  0001 C CNN
	1    2400 1050
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR0103
U 1 1 5CDD0905
P 3600 1200
F 0 "#PWR0103" H 3600 1050 50  0001 C CNN
F 1 "+3.3V" V 3615 1328 50  0000 L CNN
F 2 "" H 3600 1200 50  0001 C CNN
F 3 "" H 3600 1200 50  0001 C CNN
	1    3600 1200
	0    1    1    0   
$EndComp
$Comp
L Sensor:DHT11 U2
U 1 1 5CDCE26F
P 3300 1200
F 0 "U2" H 3056 1246 50  0000 R CNN
F 1 "DHT22" H 3056 1155 50  0000 R CNN
F 2 "Sensor:Aosong_DHT11_5.5x12.0_P2.54mm" H 3300 800 50  0001 C CNN
F 3 "http://akizukidenshi.com/download/ds/aosong/DHT11.pdf" H 3450 1450 50  0001 C CNN
	1    3300 1200
	0    1    1    0   
$EndComp
Wire Wire Line
	2650 2250 2900 2250
Text GLabel 2950 2950 2    50   Input ~ 0
dht_data
Wire Wire Line
	2650 2950 2950 2950
Text GLabel 3200 1800 0    50   Input ~ 0
dht_data
Wire Wire Line
	3200 1800 3300 1800
Connection ~ 3300 1800
$Comp
L Device:C C2
U 1 1 5CE6EEAB
P 4500 2450
F 0 "C2" H 4615 2496 50  0000 L CNN
F 1 "100nF" H 4615 2405 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P5.00mm" H 4538 2300 50  0001 C CNN
F 3 "~" H 4500 2450 50  0001 C CNN
	1    4500 2450
	1    0    0    -1  
$EndComp
Connection ~ 4500 2300
Wire Wire Line
	4500 2300 4550 2300
Wire Wire Line
	4350 2300 4500 2300
$Comp
L power:GND #PWR0120
U 1 1 5CE71516
P 4500 2600
F 0 "#PWR0120" H 4500 2350 50  0001 C CNN
F 1 "GND" H 4505 2427 50  0000 C CNN
F 2 "" H 4500 2600 50  0001 C CNN
F 3 "" H 4500 2600 50  0001 C CNN
	1    4500 2600
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR01
U 1 1 5CEA2330
P 8400 2700
F 0 "#PWR01" H 8400 2450 50  0001 C CNN
F 1 "GND" H 8405 2527 50  0000 C CNN
F 2 "" H 8400 2700 50  0001 C CNN
F 3 "" H 8400 2700 50  0001 C CNN
F 4 "I" H 8400 2700 50  0001 C CNN "Spice_Primitive"
F 5 "dc 0" H 8400 2700 50  0001 C CNN "Spice_Model"
F 6 "Y" H 8400 2700 50  0001 C CNN "Spice_Netlist_Enabled"
	1    8400 2700
	1    0    0    -1  
$EndComp
$Comp
L Device:R R9
U 1 1 5CEA2ED1
P 8400 2300
F 0 "R9" H 8470 2346 50  0000 L CNN
F 1 "10k" H 8470 2255 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 8330 2300 50  0001 C CNN
F 3 "~" H 8400 2300 50  0001 C CNN
	1    8400 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	8850 2250 8850 2700
Wire Wire Line
	8250 2050 8400 2050
Wire Wire Line
	8400 2150 8400 2050
Connection ~ 8400 2050
Wire Wire Line
	8400 2050 8550 2050
Wire Wire Line
	8400 2450 8400 2700
$EndSCHEMATC
