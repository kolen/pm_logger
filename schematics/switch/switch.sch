EESchema Schematic File Version 4
LIBS:switch-cache
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
L Transistor_FET:IRF9540N Q2
U 1 1 5CDE8BA4
P 5950 6400
F 0 "Q2" H 6156 6354 50  0000 L CNN
F 1 "IRF9510" H 6156 6445 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-220-3_Vertical" H 6150 6325 50  0001 L CIN
F 3 "http://www.irf.com/product-info/datasheets/data/irf9540n.pdf" H 5950 6400 50  0001 L CNN
F 4 "X" H 5950 6400 50  0001 C CNN "Spice_Primitive"
F 5 "irf9510" H 5950 6400 50  0001 C CNN "Spice_Model"
F 6 "Y" H 5950 6400 50  0001 C CNN "Spice_Netlist_Enabled"
F 7 "/Users/kolen/items/pm_sensor/schematics/sihf9510.lib" H 5950 6400 50  0001 C CNN "Spice_Lib_File"
F 8 "2 1 3" H 5950 6400 50  0001 C CNN "Spice_Node_Sequence"
	1    5950 6400
	1    0    0    1   
$EndComp
$Comp
L Device:R R2
U 1 1 5CDECCD9
P 5550 6150
F 0 "R2" H 5620 6196 50  0000 L CNN
F 1 "10k" H 5620 6105 50  0000 L CNN
F 2 "" V 5480 6150 50  0001 C CNN
F 3 "~" H 5550 6150 50  0001 C CNN
	1    5550 6150
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5CDF1575
P 5550 7500
F 0 "#PWR02" H 5550 7250 50  0001 C CNN
F 1 "GND" H 5555 7327 50  0000 C CNN
F 2 "" H 5550 7500 50  0001 C CNN
F 3 "" H 5550 7500 50  0001 C CNN
F 4 "I" H 5550 7500 50  0001 C CNN "Spice_Primitive"
F 5 "dc 0" H 5550 7500 50  0001 C CNN "Spice_Model"
F 6 "Y" H 5550 7500 50  0001 C CNN "Spice_Netlist_Enabled"
	1    5550 7500
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 7400 5550 7500
$Comp
L Device:R RLoad1
U 1 1 5CDF20A3
P 6050 6900
F 0 "RLoad1" H 6120 6946 50  0000 L CNN
F 1 "500" H 6120 6855 50  0000 L CNN
F 2 "" V 5980 6900 50  0001 C CNN
F 3 "~" H 6050 6900 50  0001 C CNN
	1    6050 6900
	1    0    0    -1  
$EndComp
Text GLabel 5050 5900 1    50   Output ~ 0
pwr
$Comp
L pspice:VSOURCE V0
U 1 1 5CE03A2A
P 4500 5900
F 0 "V0" V 4865 5900 50  0000 C CNN
F 1 "5" V 4774 5900 50  0000 C CNN
F 2 "" H 4500 5900 50  0001 C CNN
F 3 "~" H 4500 5900 50  0001 C CNN
	1    4500 5900
	0    1    -1   0   
$EndComp
$Comp
L power:GND #PWR01
U 1 1 5CE0567D
P 4200 5900
F 0 "#PWR01" H 4200 5650 50  0001 C CNN
F 1 "GND" H 4205 5727 50  0000 C CNN
F 2 "" H 4200 5900 50  0001 C CNN
F 3 "" H 4200 5900 50  0001 C CNN
	1    4200 5900
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 6400 5750 6400
Wire Wire Line
	6050 6600 6050 6700
Wire Wire Line
	6050 5900 6050 6200
Wire Wire Line
	4800 5900 5550 5900
Wire Wire Line
	5550 6300 5550 6400
Wire Wire Line
	5550 6000 5550 5900
Connection ~ 5550 5900
Wire Wire Line
	5550 5900 6050 5900
Wire Wire Line
	5550 7400 6050 7400
Wire Wire Line
	6050 7400 6050 7050
Wire Wire Line
	6050 6700 6450 6700
Connection ~ 6050 6700
Wire Wire Line
	6050 6700 6050 6750
Text GLabel 6550 6700 2    50   Output ~ 0
load_v
$Comp
L pspice:VSOURCE V1
U 1 1 5CE12495
P 4450 7000
F 0 "V1" V 4815 7000 50  0000 C CNN
F 1 "5" V 4724 7000 50  0000 C CNN
F 2 "" H 4450 7000 50  0001 C CNN
F 3 "~" H 4450 7000 50  0001 C CNN
F 4 "V" H 4450 7000 50  0001 C CNN "Spice_Primitive"
F 5 "pulse(0 3.3 0 1u 1u 0.5 3)" H 4450 7000 50  0001 C CNN "Spice_Model"
F 6 "Y" H 4450 7000 50  0001 C CNN "Spice_Netlist_Enabled"
	1    4450 7000
	0    1    -1   0   
$EndComp
$Comp
L power:GND #PWR0101
U 1 1 5CE13C99
P 4150 7000
F 0 "#PWR0101" H 4150 6750 50  0001 C CNN
F 1 "GND" H 4155 6827 50  0000 C CNN
F 2 "" H 4150 7000 50  0001 C CNN
F 3 "" H 4150 7000 50  0001 C CNN
	1    4150 7000
	1    0    0    -1  
$EndComp
$Comp
L Transistor_FET:2N7000 Q1
U 1 1 5CE19D61
P 5450 7000
F 0 "Q1" H 5656 7046 50  0000 L CNN
F 1 "2N7000" H 5656 6955 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline" H 5650 6925 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/2N/2N7000.pdf" H 5450 7000 50  0001 L CNN
F 4 "X" H 5450 7000 50  0001 C CNN "Spice_Primitive"
F 5 "2n7000" H 5450 7000 50  0001 C CNN "Spice_Model"
F 6 "Y" H 5450 7000 50  0001 C CNN "Spice_Netlist_Enabled"
F 7 "3 2 1" H 5450 7000 50  0001 C CNN "Spice_Node_Sequence"
F 8 "/Users/kolen/items/pm_sensor/schematics/2n7000.lib" H 5450 7000 50  0001 C CNN "Spice_Lib_File"
	1    5450 7000
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 7200 5550 7400
Connection ~ 5550 7400
Connection ~ 5550 6400
Wire Wire Line
	4750 7000 4850 7000
$Comp
L Device:R R3
U 1 1 5CE2396C
P 5000 7000
F 0 "R3" V 4793 7000 50  0000 C CNN
F 1 "300" V 4884 7000 50  0000 C CNN
F 2 "" V 4930 7000 50  0001 C CNN
F 3 "~" H 5000 7000 50  0001 C CNN
	1    5000 7000
	0    1    1    0   
$EndComp
Wire Wire Line
	5150 7000 5250 7000
Text GLabel 5250 7000 1    50   Input ~ 0
input
$Comp
L Device:C C1
U 1 1 5CDEC35A
P 6450 7100
F 0 "C1" H 6565 7146 50  0000 L CNN
F 1 "0.1u" H 6565 7055 50  0000 L CNN
F 2 "" H 6488 6950 50  0001 C CNN
F 3 "~" H 6450 7100 50  0001 C CNN
	1    6450 7100
	1    0    0    -1  
$EndComp
Wire Wire Line
	6450 6950 6450 6700
Connection ~ 6450 6700
Wire Wire Line
	6450 6700 6550 6700
Wire Wire Line
	6450 7250 6450 7400
Wire Wire Line
	6450 7400 6050 7400
Connection ~ 6050 7400
Wire Wire Line
	5550 6400 5550 6500
$Comp
L Device:R R1
U 1 1 5CDEEC41
P 5550 6650
F 0 "R1" H 5620 6696 50  0000 L CNN
F 1 "300" H 5620 6605 50  0000 L CNN
F 2 "" V 5480 6650 50  0001 C CNN
F 3 "~" H 5550 6650 50  0001 C CNN
	1    5550 6650
	1    0    0    -1  
$EndComp
$EndSCHEMATC
