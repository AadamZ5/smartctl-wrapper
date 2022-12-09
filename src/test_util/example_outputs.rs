pub const EXAMPLE_ALL: [&str; 1] = [{
    r#"
		{
			"json_format_version": [
				1,
				0
			],
			"smartctl": {
				"version": [
					7,
					2
				],
				"svn_revision": "5155",
				"platform_info": "x86_64-linux-6.0.6-76060006-generic",
				"build_info": "(local build)",
				"argv": [
					"smartctl",
					"--all",
					"-j",
					"/dev/sda"
				],
				"exit_status": 64
			},
			"device": {
				"name": "/dev/sda",
				"info_name": "/dev/sda [SAT]",
				"type": "sat",
				"protocol": "ATA"
			},
			"model_family": "Toshiba HG6 Series SSD",
			"model_name": "TOSHIBA MODELNUMBER",
			"serial_number": "SERIAL",
			"wwn": {
				"naa": 1,
				"oui": 1234,
				"id": 12345678901
			},
			"firmware_version": "FIRMWARE",
			"user_capacity": {
				"blocks": 500118192,
				"bytes": 256060514304
			},
			"logical_block_size": 512,
			"physical_block_size": 512,
			"rotation_rate": 0,
			"form_factor": {
				"ata_value": 3,
				"name": "2.5 inches"
			},
			"trim": {
				"supported": true,
				"deterministic": true,
				"zeroed": true
			},
			"in_smartctl_database": true,
			"ata_version": {
				"string": "ACS-2 (minor revision not indicated)",
				"major_value": 1016,
				"minor_value": 0
			},
			"sata_version": {
				"string": "SATA 3.1",
				"value": 127
			},
			"interface_speed": {
				"max": {
					"sata_value": 14,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				},
				"current": {
					"sata_value": 3,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				}
			},
			"local_time": {
				"time_t": 1668116567,
				"asctime": "Thu Nov 10 16:42:47 2022 EST"
			},
			"smart_status": {
				"passed": true
			},
			"ata_smart_data": {
				"offline_data_collection": {
					"status": {
						"value": 0,
						"string": "was never started"
					},
					"completion_seconds": 120
				},
				"self_test": {
					"status": {
						"value": 0,
						"string": "completed without error",
						"passed": true
					},
					"polling_minutes": {
						"short": 2,
						"extended": 10
					}
				},
				"capabilities": {
					"values": [
						91,
						3
					],
					"exec_offline_immediate_supported": true,
					"offline_is_aborted_upon_new_cmd": false,
					"offline_surface_scan_supported": true,
					"self_tests_supported": true,
					"conveyance_self_test_supported": false,
					"selective_self_test_supported": true,
					"attribute_autosave_enabled": true,
					"error_logging_supported": true,
					"gp_logging_supported": true
				}
			},
			"ata_sct_capabilities": {
				"value": 61,
				"error_recovery_control_supported": true,
				"feature_control_supported": true,
				"data_table_supported": true
			},
			"ata_smart_attributes": {
				"revision": 16,
				"table": [
					{
						"id": 1,
						"name": "Raw_Read_Error_Rate",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 10,
							"string": "-O-R-- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": true,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 2,
						"name": "Throughput_Performance",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 5,
							"string": "P-S--- ",
							"prefailure": true,
							"updated_online": false,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 3,
						"name": "Spin_Up_Time",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 7,
							"string": "POS--- ",
							"prefailure": true,
							"updated_online": true,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 5,
						"name": "Reallocated_Sector_Ct",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 7,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 11,
							"string": "PO-R-- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": true,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 8,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 5,
							"string": "P-S--- ",
							"prefailure": true,
							"updated_online": false,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 9,
						"name": "Power_On_Hours",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 60323,
							"string": "60323"
						}
					},
					{
						"id": 10,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 12,
						"name": "Power_Cycle_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 955,
							"string": "955"
						}
					},
					{
						"id": 167,
						"name": "SSD_Protect_Mode",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 34,
							"string": "-O---K ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": false,
							"auto_keep": true
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 168,
						"name": "SATA_PHY_Error_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 4,
							"string": "4"
						}
					},
					{
						"id": 169,
						"name": "Bad_Block_Count",
						"value": 100,
						"worst": 100,
						"thresh": 10,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 100,
							"string": "100"
						}
					},
					{
						"id": 173,
						"name": "Erase_Count",
						"value": 137,
						"worst": 137,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 175,
						"name": "Program_Fail_Count_Chip",
						"value": 100,
						"worst": 100,
						"thresh": 10,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 192,
						"name": "Power-Off_Retract_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 200,
							"string": "200"
						}
					},
					{
						"id": 194,
						"name": "Temperature_Celsius",
						"value": 70,
						"worst": 63,
						"thresh": 20,
						"when_failed": "",
						"flags": {
							"value": 35,
							"string": "PO---K ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": false,
							"auto_keep": true
						},
						"raw": {
							"value": 158914576414,
							"string": "30 (Min/Max 12/37)"
						}
					},
					{
						"id": 197,
						"name": "Current_Pending_Sector",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 240,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					}
				]
			},
			"power_on_time": {
				"hours": 60323
			},
			"power_cycle_count": 955,
			"temperature": {
				"current": 30
			},
			"ata_smart_error_log": {
				"summary": {
					"revision": 1,
					"count": 3,
					"logged_count": 3,
					"table": [
						{
							"error_number": 3,
							"lifetime_hours": 39104,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 208,
								"lba": 79880,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x00013808 = 79880",
							"previous_commands": [
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 208,
										"lba": 79880,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902050,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 200,
										"lba": 79880,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902049,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 192,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902047,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 184,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902045,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 176,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902044,
									"command_name": "WRITE FPDMA QUEUED"
								}
							]
						},
						{
							"error_number": 2,
							"lifetime_hours": 33074,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 8,
								"lba": 3786544,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x0039c730 = 3786544",
							"previous_commands": [
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 8,
										"lba": 3786544,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439400,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 234,
										"features": 0,
										"count": 0,
										"lba": 0,
										"device": 160,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "FLUSH CACHE EXT"
								},
								{
									"registers": {
										"command": 97,
										"features": 16,
										"count": 240,
										"lba": 7095624,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 232,
										"lba": 3786456,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 234,
										"features": 0,
										"count": 0,
										"lba": 0,
										"device": 160,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439398,
									"command_name": "FLUSH CACHE EXT"
								}
							]
						},
						{
							"error_number": 1,
							"lifetime_hours": 23292,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 136,
								"lba": 10477504,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x009fdfc0 = 10477504",
							"previous_commands": [
								{
									"registers": {
										"command": 96,
										"features": 178,
										"count": 152,
										"lba": 10478016,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 0,
										"count": 144,
										"lba": 10477760,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 0,
										"count": 136,
										"lba": 10477504,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 8,
										"count": 128,
										"lba": 15414720,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50254,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 200,
										"count": 120,
										"lba": 4377496,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50099,
									"command_name": "READ FPDMA QUEUED"
								}
							]
						}
					]
				}
			},
			"ata_smart_self_test_log": {
				"standard": {
					"revision": 1,
					"table": [
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 53133
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 52652
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 50318
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 45543
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 40438
						},
						{
							"type": {
								"value": 1,
								"string": "Short offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 40438
						}
					],
					"count": 6,
					"error_count_total": 0,
					"error_count_outdated": 0
				}
			},
			"ata_smart_selective_self_test_log": {
				"revision": 1,
				"table": [
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 0,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 0,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 0,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 0,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 0,
							"string": "Not_testing"
						}
					}
				],
				"flags": {
					"value": 0,
					"remainder_scan_enabled": false
				},
				"power_up_scan_resume_minutes": 0
			}
		}
		"#
}];

pub const EXAMPLE_ALL_DURING_TESTING: [&str; 1] = [{
    r#"
		{
			"json_format_version": [
				1,
				0
			],
			"smartctl": {
				"version": [
					7,
					2
				],
				"svn_revision": "5155",
				"platform_info": "x86_64-linux-6.0.6-76060006-generic",
				"build_info": "(local build)",
				"argv": [
					"smartctl",
					"-j",
					"--all",
					"/dev/sda"
				],
				"exit_status": 64
			},
			"device": {
				"name": "/dev/sda",
				"info_name": "/dev/sda [SAT]",
				"type": "sat",
				"protocol": "ATA"
			},
			"model_family": "Toshiba HG6 Series SSD",
			"model_name": "TOSHIBA MODELNUMBER",
			"serial_number": "SERIAL",
			"wwn": {
				"naa": 1,
				"oui": 1234,
				"id": 12345678901
			},
			"firmware_version": "FIRMWARE",
			"user_capacity": {
				"blocks": 500118192,
				"bytes": 256060514304
			},
			"logical_block_size": 512,
			"physical_block_size": 512,
			"rotation_rate": 0,
			"form_factor": {
				"ata_value": 3,
				"name": "2.5 inches"
			},
			"trim": {
				"supported": true,
				"deterministic": true,
				"zeroed": true
			},
			"in_smartctl_database": true,
			"ata_version": {
				"string": "ACS-2 (minor revision not indicated)",
				"major_value": 1016,
				"minor_value": 0
			},
			"sata_version": {
				"string": "SATA 3.1",
				"value": 127
			},
			"interface_speed": {
				"max": {
					"sata_value": 14,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				},
				"current": {
					"sata_value": 3,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				}
			},
			"local_time": {
				"time_t": 1670612862,
				"asctime": "Fri Dec  9 14:07:42 2022 EST"
			},
			"smart_status": {
				"passed": true
			},
			"ata_smart_data": {
				"offline_data_collection": {
					"status": {
						"value": 0,
						"string": "was never started"
					},
					"completion_seconds": 120
				},
				"self_test": {
					"status": {
						"value": 249,
						"string": "in progress, 90% remaining",
						"remaining_percent": 90
					},
					"polling_minutes": {
						"short": 2,
						"extended": 10
					}
				},
				"capabilities": {
					"values": [
						91,
						3
					],
					"exec_offline_immediate_supported": true,
					"offline_is_aborted_upon_new_cmd": false,
					"offline_surface_scan_supported": true,
					"self_tests_supported": true,
					"conveyance_self_test_supported": false,
					"selective_self_test_supported": true,
					"attribute_autosave_enabled": true,
					"error_logging_supported": true,
					"gp_logging_supported": true
				}
			},
			"ata_sct_capabilities": {
				"value": 61,
				"error_recovery_control_supported": true,
				"feature_control_supported": true,
				"data_table_supported": true
			},
			"ata_smart_attributes": {
				"revision": 16,
				"table": [
					{
						"id": 1,
						"name": "Raw_Read_Error_Rate",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 10,
							"string": "-O-R-- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": true,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 2,
						"name": "Throughput_Performance",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 5,
							"string": "P-S--- ",
							"prefailure": true,
							"updated_online": false,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 3,
						"name": "Spin_Up_Time",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 7,
							"string": "POS--- ",
							"prefailure": true,
							"updated_online": true,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 5,
						"name": "Reallocated_Sector_Ct",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 7,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 11,
							"string": "PO-R-- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": true,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 8,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 5,
							"string": "P-S--- ",
							"prefailure": true,
							"updated_online": false,
							"performance": true,
							"error_rate": false,
							"event_count": false,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 9,
						"name": "Power_On_Hours",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 61016,
							"string": "61016"
						}
					},
					{
						"id": 10,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 12,
						"name": "Power_Cycle_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 955,
							"string": "955"
						}
					},
					{
						"id": 167,
						"name": "SSD_Protect_Mode",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 34,
							"string": "-O---K ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": false,
							"auto_keep": true
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 168,
						"name": "SATA_PHY_Error_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 4,
							"string": "4"
						}
					},
					{
						"id": 169,
						"name": "Bad_Block_Count",
						"value": 100,
						"worst": 100,
						"thresh": 10,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 100,
							"string": "100"
						}
					},
					{
						"id": 173,
						"name": "Erase_Count",
						"value": 135,
						"worst": 135,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 175,
						"name": "Program_Fail_Count_Chip",
						"value": 100,
						"worst": 100,
						"thresh": 10,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 192,
						"name": "Power-Off_Retract_Count",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 200,
							"string": "200"
						}
					},
					{
						"id": 194,
						"name": "Temperature_Celsius",
						"value": 70,
						"worst": 63,
						"thresh": 20,
						"when_failed": "",
						"flags": {
							"value": 35,
							"string": "PO---K ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": false,
							"auto_keep": true
						},
						"raw": {
							"value": 158914576414,
							"string": "30 (Min/Max 12/37)"
						}
					},
					{
						"id": 197,
						"name": "Current_Pending_Sector",
						"value": 100,
						"worst": 100,
						"thresh": 0,
						"when_failed": "",
						"flags": {
							"value": 18,
							"string": "-O--C- ",
							"prefailure": false,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					},
					{
						"id": 240,
						"name": "Unknown_SSD_Attribute",
						"value": 100,
						"worst": 100,
						"thresh": 50,
						"when_failed": "",
						"flags": {
							"value": 19,
							"string": "PO--C- ",
							"prefailure": true,
							"updated_online": true,
							"performance": false,
							"error_rate": false,
							"event_count": true,
							"auto_keep": false
						},
						"raw": {
							"value": 0,
							"string": "0"
						}
					}
				]
			},
			"power_on_time": {
				"hours": 61016
			},
			"power_cycle_count": 955,
			"temperature": {
				"current": 30
			},
			"ata_smart_error_log": {
				"summary": {
					"revision": 1,
					"count": 3,
					"logged_count": 3,
					"table": [
						{
							"error_number": 3,
							"lifetime_hours": 39104,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 208,
								"lba": 79880,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x00013808 = 79880",
							"previous_commands": [
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 208,
										"lba": 79880,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902050,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 200,
										"lba": 79880,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902049,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 192,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902047,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 184,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902045,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 176,
										"lba": 79792,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 2158902044,
									"command_name": "WRITE FPDMA QUEUED"
								}
							]
						},
						{
							"error_number": 2,
							"lifetime_hours": 33074,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 8,
								"lba": 3786544,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x0039c730 = 3786544",
							"previous_commands": [
								{
									"registers": {
										"command": 97,
										"features": 96,
										"count": 8,
										"lba": 3786544,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439400,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 234,
										"features": 0,
										"count": 0,
										"lba": 0,
										"device": 160,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "FLUSH CACHE EXT"
								},
								{
									"registers": {
										"command": 97,
										"features": 16,
										"count": 240,
										"lba": 7095624,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 97,
										"features": 88,
										"count": 232,
										"lba": 3786456,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439399,
									"command_name": "WRITE FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 234,
										"features": 0,
										"count": 0,
										"lba": 0,
										"device": 160,
										"device_control": 0
									},
									"powerup_milliseconds": 1101439398,
									"command_name": "FLUSH CACHE EXT"
								}
							]
						},
						{
							"error_number": 1,
							"lifetime_hours": 23292,
							"completion_registers": {
								"error": 132,
								"status": 81,
								"count": 136,
								"lba": 10477504,
								"device": 64
							},
							"error_description": "Error: ICRC, ABRT at LBA = 0x009fdfc0 = 10477504",
							"previous_commands": [
								{
									"registers": {
										"command": 96,
										"features": 178,
										"count": 152,
										"lba": 10478016,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 0,
										"count": 144,
										"lba": 10477760,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 0,
										"count": 136,
										"lba": 10477504,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50326,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 8,
										"count": 128,
										"lba": 15414720,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50254,
									"command_name": "READ FPDMA QUEUED"
								},
								{
									"registers": {
										"command": 96,
										"features": 200,
										"count": 120,
										"lba": 4377496,
										"device": 64,
										"device_control": 0
									},
									"powerup_milliseconds": 50099,
									"command_name": "READ FPDMA QUEUED"
								}
							]
						}
					]
				}
			},
			"ata_smart_self_test_log": {
				"standard": {
					"revision": 1,
					"table": [
						{
							"type": {
								"value": 1,
								"string": "Short offline"
							},
							"status": {
								"value": 16,
								"string": "Aborted by host"
							},
							"lifetime_hours": 61016
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 53133
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 52652
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 50318
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 45543
						},
						{
							"type": {
								"value": 2,
								"string": "Extended offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 40438
						},
						{
							"type": {
								"value": 1,
								"string": "Short offline"
							},
							"status": {
								"value": 0,
								"string": "Completed without error",
								"passed": true
							},
							"lifetime_hours": 40438
						}
					],
					"count": 7,
					"error_count_total": 0,
					"error_count_outdated": 0
				}
			},
			"ata_smart_selective_self_test_log": {
				"revision": 1,
				"table": [
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 249,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 249,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 249,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 249,
							"string": "Not_testing"
						}
					},
					{
						"lba_min": 0,
						"lba_max": 0,
						"status": {
							"value": 249,
							"string": "Not_testing"
						}
					}
				],
				"flags": {
					"value": 0,
					"remainder_scan_enabled": false
				},
				"power_up_scan_resume_minutes": 0
			}
		}
		"#
}];

pub const EXAMPLE_INFO: [&str; 1] = [{
    r#"
		{
			"json_format_version": [
				1,
				0
			],
			"smartctl": {
				"version": [
					7,
					2
				],
				"svn_revision": "5155",
				"platform_info": "x86_64-linux-6.0.6-76060006-generic",
				"build_info": "(local build)",
				"argv": [
					"smartctl",
					"--info",
					"-j",
					"/dev/sda"
				],
				"exit_status": 0
			},
			"device": {
				"name": "/dev/sda",
				"info_name": "/dev/sda [SAT]",
				"type": "sat",
				"protocol": "ATA"
			},
			"model_family": "Toshiba HG6 Series SSD",
			"model_name": "TOSHIBA MODELNUMBER",
			"serial_number": "SERIAL",
			"wwn": {
				"naa": 1,
				"oui": 1234,
				"id": 12345678901
			},
			"firmware_version": "FIRMWARE",
			"user_capacity": {
				"blocks": 500118192,
				"bytes": 256060514304
			},
			"logical_block_size": 512,
			"physical_block_size": 512,
			"rotation_rate": 0,
			"form_factor": {
				"ata_value": 3,
				"name": "2.5 inches"
			},
			"trim": {
				"supported": true,
				"deterministic": true,
				"zeroed": true
			},
			"in_smartctl_database": true,
			"ata_version": {
				"string": "ACS-2 (minor revision not indicated)",
				"major_value": 1016,
				"minor_value": 0
			},
			"sata_version": {
				"string": "SATA 3.1",
				"value": 127
			},
			"interface_speed": {
				"max": {
					"sata_value": 14,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				},
				"current": {
					"sata_value": 3,
					"string": "6.0 Gb/s",
					"units_per_second": 60,
					"bits_per_unit": 100000000
				}
			},
			"local_time": {
				"time_t": 1668116616,
				"asctime": "Thu Nov 10 16:43:36 2022 EST"
			}
		}
		"#
}];

pub const EXAMPLE_SCAN: [&str; 1] = [{
    r#"
		{
			"json_format_version": [
				1,
				0
			],
			"smartctl": {
				"version": [
					7,
					2
				],
				"svn_revision": "5155",
				"platform_info": "x86_64-linux-6.0.6-76060006-generic",
				"build_info": "(local build)",
				"argv": [
					"smartctl",
					"--scan",
					"-j"
				],
				"exit_status": 0
			},
			"devices": [
				{
					"name": "/dev/sda",
					"info_name": "/dev/sda",
					"type": "scsi",
					"protocol": "SCSI"
				},
				{
					"name": "/dev/sdb",
					"info_name": "/dev/sdb",
					"type": "scsi",
					"protocol": "SCSI"
				},
				{
					"name": "/dev/sdc",
					"info_name": "/dev/sdc",
					"type": "scsi",
					"protocol": "SCSI"
				}
			]
		}
		"#
}];

pub const EXAMPLE_SCAN_OPEN: [&str; 1] = [{
    r#"
		{
			"json_format_version": [
				1,
				0
			],
			"smartctl": {
				"version": [
					7,
					2
				],
				"svn_revision": "5155",
				"platform_info": "x86_64-linux-6.0.6-76060006-generic",
				"build_info": "(local build)",
				"argv": [
					"smartctl",
					"--scan-open",
					"-j"
				],
				"exit_status": 0
			},
			"devices": [
				{
					"name": "/dev/sda",
					"info_name": "/dev/sda [SAT]",
					"type": "sat",
					"protocol": "ATA"
				},
				{
					"name": "/dev/sdb",
					"info_name": "/dev/sdb [SAT]",
					"type": "sat",
					"protocol": "ATA"
				},
				{
					"name": "/dev/sdc",
					"info_name": "/dev/sdc [SAT]",
					"type": "sat",
					"protocol": "ATA"
				}
			]
		}
		"#
}];

pub mod output_pieces {
    pub const TEST_ENTRY: [&str; 1] = [{
        r#"
			{
				"type": {
					"value": 2,
					"string": "Extended offline"
				},
				"status": {
					"value": 0,
					"string": "Completed without error",
					"passed": true
				},
				"lifetime_hours": 53133
			}
			"#
    }];
}
