import 'dart:convert';
import 'dart:io';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:flutter/cupertino.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_hbb/common.dart';


// Device Manager
class Device {
  Device._privateConstructor();

  /// cache uniqueId
  //static String? _uniqueId;

  /// A unique device identifier.
  ///
  /// Refer[Unity deviceUniqueIdentifier](https://docs.unity3d.com/ScriptReference/SystemInfo-deviceUniqueIdentifier.html)
  static Future<String> uniqueIdentifier() async {

    // fetch ids in windows 
    final baseBoardID = await _winBaseBoardID();
    final biosID = await _winBiosID();
    final processorID = await _winProcessorID();
    final diskDriveID = await _winDiskDrive();
    //final osNumber = await _winOSNumber();
    // md5 generates a unique id, using String.hashCode directly is too easy to collide
    final all = baseBoardID + biosID + processorID + diskDriveID;// + osNumber;
    final uID = md5.convert(utf8.encode(all)).toString();
    debugPrint('baseBoard: $baseBoardID biosID: $biosID '
        'processorID: $processorID diskDriveID: $diskDriveID '
        'uID: $uID');
    
    return uID;
  }
  /// windows `Win32_BaseBoard::SerialNumber`
  ///
  /// cmd: `wmic baseboard get SerialNumber`
  static Future<String> _winBaseBoardID() async {
    return _fetchWinID(
      'wmic',
      ['baseboard', 'get', 'serialnumber'],
      'serialnumber',
    );
  }

  /// windows `Win32_BIOS::SerialNumber`
  ///
  /// cmd: `wmic csproduct get UUID`
  static Future<String> _winBiosID() async {
    return _fetchWinID(
      'wmic',
      ['csproduct', 'get', 'uuid'],
      'uuid',
    );
  }

  /// windows `Win32_Processor::UniqueId`
  ///
  /// cmd: `wmic baseboard get SerialNumber`
  static Future<String> _winProcessorID() async {
    return _fetchWinID(
      'wmic',
      ['cpu', 'get', 'processorid'],
      'processorid',
    );
  }

  /// windows `Win32_DiskDrive::SerialNumber`
  ///
  /// cmd: `wmic diskdrive get SerialNumber`
  static Future<String> _winDiskDrive() async {
    return _fetchWinID(
      'wmic',
      ['diskdrive', 'get', 'serialnumber'],
      'serialnumber',
    );
  }

  /// windows `Win32_OperatingSystem::SerialNumber`
  ///
  /// cmd: `wmic os get serialnumber`
  static Future<String> _winOSNumber() async {
    return _fetchWinID(
      'wmic',
      ['os', 'get', 'serialnumber'],
      'serialnumber',
    );
  }

  static Future<String?> GetDeviceName() async{
    String? deviceName;
    if (isDesktop){
       deviceName = Platform.localHostname;
    }else{
      try{
        // first, check if bluetooth is supported by your hardware
// Note: The platform is initialized on the first call to any FlutterBluePlus method.
        if (await FlutterBluePlus.isSupported == false) {
            print("Bluetooth not supported by this device");
            return '';
        }

        // handle bluetooth on & off
        // note: for iOS the initial state is typically BluetoothAdapterState.unknown
        // note: if you have permissions issues you will get stuck at BluetoothAdapterState.unauthorized
        var subscription = FlutterBluePlus.adapterState.listen((BluetoothAdapterState state) {
            print(state);
            if (state == BluetoothAdapterState.on) {
            deviceName = FlutterBluePlus.adapterName as String;
            
                // show an error to the user, etc
            }
        });

        // turn on bluetooth ourself if we can
        // for iOS, the user controls bluetooth enable/disable
        if (Platform.isAndroid) {
            await FlutterBluePlus.turnOn();
        }
        subscription.cancel();
        
      } catch (e) {
        deviceName = e.toString();
      }
    } 
    return deviceName;
  }
  /// fetch windows id by cmd line
  static Future<String> _fetchWinID(
    String executable,
    List<String> arguments,
    String regExpSource,
  ) async {
    String id = '';
    try {
      final process = await Process.start(
        executable,
        arguments,
        mode: ProcessStartMode.detachedWithStdio,
      );
      final result = await process.stdout.transform(utf8.decoder).toList();
      for (var element in result) {
        final item = element.toLowerCase().replaceAll(
              RegExp('\r|\n|\\s|$regExpSource'),
              '',
            );
        if (item.isNotEmpty) {
          id = id + item;
        }
      }
    } on Exception catch (_) {}
    return id;
  }
}