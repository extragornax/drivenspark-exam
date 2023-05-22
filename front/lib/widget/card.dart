import 'package:flutter/material.dart';

/// Flutter code sample for [Card].

// void main() => runApp(const CardExampleApp());

// class CardExampleApp extends StatelessWidget {
//   const CardExampleApp({super.key});

//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(title: const Text('Card Sample')),
//         body: const CardItem(),
//       ),
//     );
//   }
// }

class CardItem extends StatelessWidget {
  const CardItem(id, title, description, date, priority, duration, status,
      {Key? key})
      : id = id,
        title = title,
        description = description,
        date = date,
        priority = priority,
        duration = duration,
        status = status,
        super(key: key);

  final int id;
  final String title;
  final String description;
  final String date;
  final String priority;
  final int duration;
  final String status;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Card(
        // clipBehavior is necessary because, without it, the InkWell's animation
        // will extend beyond the rounded edges of the [Card] (see https://github.com/flutter/flutter/issues/109776)
        // This comes with a small performance cost, and you should not set [clipBehavior]
        // unless you need it.
        clipBehavior: Clip.hardEdge,
        child: InkWell(
          splashColor: Colors.blue.withAlpha(30),
          onTap: () {
            debugPrint('Card tapped.');
          },
          child: const SizedBox(
            width: 300,
            height: 100,
            child: DataItem(
                id, title, description, date, priority, duration, status),
          ),
        ),
      ),
    );
  }
}
