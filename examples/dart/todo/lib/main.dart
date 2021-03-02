import 'generated/rid_generated.dart' as ffi;

main() {
  final model = ffi.rid_ffi.init_model_ptr(111);
  print("""
model id: ${model.id}
ids len:  ${model.ids.length}
ids:      [ ${model.ids[0]}, ${model.ids[1]},  ${model.ids[2]} ]
""");

  for (int id in model.ids.iter()) {
    print("id: $id");
  }

  print("out of range: [ ${model.ids[3]} ]");
}
