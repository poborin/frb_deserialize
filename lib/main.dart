import 'package:deserialize/src/rust/api/training_plan.dart';
import 'package:flutter/material.dart';
import 'package:deserialize/src/rust/frb_generated.dart';
import 'package:flutter/services.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

Future<String?> _loadTestData() async {
  try {
    final jsonString =
        await rootBundle.loadString('assets/test_data/training_plan.json');
    print("<!> $jsonString");

    // print("<test paninc>");
    // TrainingPlan.testPanic();

    final trainingPlan = TrainingPlan.testDeserialize(content: jsonString);
    return trainingPlan.title;
  } catch (e) {
    print('Error loading test data: $e');
    return null;
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    // final trainingPlan = await _loadTestData();
    // print(trainingPlan?.title!);

    return FutureBuilder<String?>(
      future: _loadTestData(),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        } else if (snapshot.hasError || !snapshot.hasData) {
          return const Center(child: Text('Error loading test data'));
        } else {
          return Center(child: Text(snapshot.data ?? "no title"));
        }
      },
    );
  }
}
