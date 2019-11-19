{
    "id": "c589afcd-6302-4e88-82c2-a98578d6ea86",
    "modelName": "GMExtension",
    "mvc": "1.2",
    "name": "emu_logic",
    "IncludedResources": [
        
    ],
    "androidPermissions": [
        
    ],
    "androidProps": false,
    "androidactivityinject": "",
    "androidclassname": "",
    "androidinject": "",
    "androidmanifestinject": "",
    "androidsourcedir": "",
    "author": "",
    "classname": "",
    "copyToTargets": 64,
    "date": "2019-55-17 02:11:27",
    "description": "",
    "exportToGame": true,
    "extensionName": "",
    "files": [
        {
            "id": "328b78c2-5a9b-4701-9a19-1acde39499f3",
            "modelName": "GMExtensionFile",
            "mvc": "1.0",
            "ProxyFiles": [
                
            ],
            "constants": [
                
            ],
            "copyToTargets": 64,
            "filename": "emu_logic_capi.dll",
            "final": "",
            "functions": [
                {
                    "id": "6e7bab5d-4c24-4d30-8022-13ab930cf8cc",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_last_errors()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_last_errors",
                    "returnType": 1
                },
                {
                    "id": "8a972125-9dac-4b11-aca0-3299f7996832",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        1
                    ],
                    "externalName": "",
                    "help": "emu_cpu_start(rom_path)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_start",
                    "returnType": 2
                },
                {
                    "id": "f4b1fa37-c547-47b3-8824-c99477dde262",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_cpu_stop()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_stop",
                    "returnType": 2
                },
                {
                    "id": "49b43519-302b-4079-b340-ebf769413039",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_cpu_resume()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_resume",
                    "returnType": 2
                },
                {
                    "id": "7bf59fa4-962d-4def-be9a-172604a36181",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_cpu_can_resume()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_can_resume",
                    "returnType": 2
                },
                {
                    "id": "fb339436-8092-4234-9fe9-01402d1a40c5",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_set_input(flags)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_set_input",
                    "returnType": 2
                },
                {
                    "id": "f326a0f3-5898-4aaa-ae2d-4650df4772b1",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_viewport_x()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_viewport_x",
                    "returnType": 2
                },
                {
                    "id": "87bf0ad7-69f9-4d1b-96d0-85420acfd9ff",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_viewport_y()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_viewport_y",
                    "returnType": 2
                },
                {
                    "id": "2deedb0d-d3ed-4e42-9e26-d9fd0fec848a",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_sprites_count()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_sprites_count",
                    "returnType": 2
                },
                {
                    "id": "4ff904b1-7aec-4242-b245-605857320aea",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_sprite_image(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_sprite_image",
                    "returnType": 2
                },
                {
                    "id": "f8c4ccad-0f4f-47ab-b630-456f88bd9514",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_sprite_x(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_sprite_x",
                    "returnType": 2
                },
                {
                    "id": "cc885de1-cffa-4ec1-95ce-d914f035ef03",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_sprite_y(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_sprite_y",
                    "returnType": 2
                },
                {
                    "id": "5aaddde2-c37f-4349-937a-a63beb149ed3",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_manifest_images_atlas_path()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_images_atlas_path",
                    "returnType": 1
                },
                {
                    "id": "b0d0effe-a648-4451-9811-b3b132ce8708",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_manifest_images_count()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_images_count",
                    "returnType": 2
                },
                {
                    "id": "ac708cb0-5a3d-459c-a070-79d010e398cc",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_x(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_x",
                    "returnType": 2
                },
                {
                    "id": "1ab5ad18-474e-4a05-be17-8f101ff57385",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_y(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_y",
                    "returnType": 2
                },
                {
                    "id": "5f5112d0-ea70-4eae-9ae1-99f484641536",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_w(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_w",
                    "returnType": 2
                },
                {
                    "id": "174705ee-7c35-4a91-9cfa-8e8413756e7b",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_h(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_h",
                    "returnType": 2
                },
                {
                    "id": "a921aad1-b3a5-4299-85c5-f4ecdd345625",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_ox(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_ox",
                    "returnType": 2
                },
                {
                    "id": "9d64e158-b181-408e-98e9-165735fbe7b9",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        2
                    ],
                    "externalName": "",
                    "help": "emu_manifest_image_oy(index)",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_manifest_image_oy",
                    "returnType": 2
                },
                {
                    "id": "f7f53842-35f2-410e-8bcc-8f6453249d14",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_cpu_is_halt()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_is_halt",
                    "returnType": 2
                },
                {
                    "id": "004911ab-3e59-4fd0-a7f3-5ed476ebbadd",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_cpu_clear_halt()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_cpu_clear_halt",
                    "returnType": 2
                },
                {
                    "id": "c7087028-2a4e-4bf4-b38e-6b36c4fc9e6e",
                    "modelName": "GMExtensionFunction",
                    "mvc": "1.0",
                    "argCount": 0,
                    "args": [
                        
                    ],
                    "externalName": "",
                    "help": "emu_bgcolor()",
                    "hidden": false,
                    "kind": 1,
                    "name": "emu_bgcolor",
                    "returnType": 2
                }
            ],
            "init": "",
            "kind": 1,
            "order": [
                "6e7bab5d-4c24-4d30-8022-13ab930cf8cc",
                "8a972125-9dac-4b11-aca0-3299f7996832",
                "f4b1fa37-c547-47b3-8824-c99477dde262",
                "49b43519-302b-4079-b340-ebf769413039",
                "7bf59fa4-962d-4def-be9a-172604a36181",
                "fb339436-8092-4234-9fe9-01402d1a40c5",
                "f326a0f3-5898-4aaa-ae2d-4650df4772b1",
                "87bf0ad7-69f9-4d1b-96d0-85420acfd9ff",
                "2deedb0d-d3ed-4e42-9e26-d9fd0fec848a",
                "4ff904b1-7aec-4242-b245-605857320aea",
                "f8c4ccad-0f4f-47ab-b630-456f88bd9514",
                "cc885de1-cffa-4ec1-95ce-d914f035ef03",
                "5aaddde2-c37f-4349-937a-a63beb149ed3",
                "b0d0effe-a648-4451-9811-b3b132ce8708",
                "ac708cb0-5a3d-459c-a070-79d010e398cc",
                "1ab5ad18-474e-4a05-be17-8f101ff57385",
                "5f5112d0-ea70-4eae-9ae1-99f484641536",
                "174705ee-7c35-4a91-9cfa-8e8413756e7b",
                "a921aad1-b3a5-4299-85c5-f4ecdd345625",
                "9d64e158-b181-408e-98e9-165735fbe7b9",
                "f7f53842-35f2-410e-8bcc-8f6453249d14",
                "004911ab-3e59-4fd0-a7f3-5ed476ebbadd",
                "c7087028-2a4e-4bf4-b38e-6b36c4fc9e6e"
            ],
            "origname": "",
            "uncompress": false
        }
    ],
    "gradleinject": "",
    "helpfile": "",
    "installdir": "",
    "iosProps": false,
    "iosSystemFrameworkEntries": [
        
    ],
    "iosThirdPartyFrameworkEntries": [
        
    ],
    "iosdelegatename": "",
    "iosplistinject": "",
    "license": "",
    "maccompilerflags": "",
    "maclinkerflags": "",
    "macsourcedir": "",
    "options": null,
    "optionsFile": "options.json",
    "packageID": "",
    "productID": "",
    "sourcedir": "",
    "supportedTargets": 64,
    "tvosProps": false,
    "tvosSystemFrameworkEntries": [
        
    ],
    "tvosThirdPartyFrameworkEntries": [
        
    ],
    "tvosclassname": "",
    "tvosdelegatename": "",
    "tvosmaccompilerflags": "",
    "tvosmaclinkerflags": "",
    "tvosplistinject": "",
    "version": "0.1.0"
}