export interface ApAssocTestResult { 
	elapsed_time_seconds: number;
	json_extra: null | number;
	test_type_code: AP_ASSOC;
 
}

export enum AP_ASSOC{ 
	AP_ASSOC = "AP_ASSOC",
	UNREACHABLE = "UNREACHABLE" 
}