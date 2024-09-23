import 'dart:convert';
import 'dart:io';
import 'package:flutter/cupertino.dart';
import 'package:crypto/crypto.dart';

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
    final all = baseBoardID + biosID + processorID + diskDriveID; // + osNumber;
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
