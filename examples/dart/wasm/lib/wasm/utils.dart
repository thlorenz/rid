import 'dart:html';
import 'dart:typed_data';

import 'package:universal_io/io.dart';

String ROOT_URL = window.location.origin;
Future<Uint8List> loadWasmFile(String wasmFile) async {
  final path = '$ROOT_URL/$wasmFile';
  try {
    // http-server --cors
    final httpClient = HttpClient();
    final request = await httpClient.getUrl(Uri.parse(path));
    if (request is BrowserHttpClientRequest) {
      request.browserResponseType = 'arraybuffer';
    }
    final response = await request.close();
    final list = await response.toList().then((List<List<int>> lists) {
      return lists.fold<List<int>>(<int>[], (List<int> acc, List<int> list) {
        acc.addAll(list);
        return acc;
      });
    });
    return Uint8List.fromList(list);
  } catch (e) {
    print(e);
    print("Couldn't open $path");
    return Uint8List.fromList([]);
  }
}
