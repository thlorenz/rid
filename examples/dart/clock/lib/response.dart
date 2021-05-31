import 'response_channel.dart';

enum Post {
  Started,
  Stopped,
  Reset,
  Tick,
}

class Response extends IResponse {
  final Post post;
  final int id;

  Response(this.post, this.id);

  @override
  String toString() {
    return '''Response {
  post: ${this.post.toString().substring('Post.'.length)}
  id:    $id
}
''';
  }
}

const int _POST_MASK = 0x000000000000ffff;
const int _I64_MIN = -9223372036854775808;

Response decode(int packed) {
  final ntopic = packed & _POST_MASK;
  final id = (packed - _I64_MIN) >> 16;

  final post = Post.values[ntopic];
  return Response(post, id);
}

final ResponseChannel<Response> responseChannel =
    ResponseChannel.instance(decode);
