import 'package:flutter/material.dart';
import './services/data_service.dart';
import './widget/card.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await DataService.loadData();
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Trello title Gaspard W',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: DragAndDropScreen(),
    );
  }
}

class DragAndDropScreen extends StatefulWidget {
  @override
  _DragAndDropScreenState createState() => _DragAndDropScreenState();
}

class _DragAndDropScreenState extends State<DragAndDropScreen> {
  List<String> todoColumnItems = ['Item 1', 'Item 2', 'Item 3'];
  List<String> inProgressColumnItems = ['Test2'];
  List<String> completeColumnItems = [];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Trello like Gaspard W'),
      ),
      body: Row(
        children: [
          buildColumn(todoColumnItems),
          buildColumn(inProgressColumnItems),
          buildColumn(completeColumnItems),
        ],
      ),
    );
  }

  Widget buildColumn(List<String> items) {
    return Expanded(
      child: DragTarget<String>(
        builder: (BuildContext context, List<String?> candidateData,
            List<dynamic> rejectedData) {
          return ListView.builder(
            itemCount: items.length,
            itemBuilder: (BuildContext context, int index) {
              // log to console the index
              print(index);
              print(items[index]);
              final item = items[index];
              return Draggable<String>(
                data: item,
                feedback: ListTile(
                  title: Text(item),
                  // You can customize the appearance of the dragged item here
                  // For example, you can give it a different background color or opacity
                  // to indicate that it's being dragged.
                  tileColor: Colors.grey[300],
                ),
                childWhenDragging: Container(),
                child: ListTile(
                  title: Text(item),
                ),
              );
            },
          );
        },
        onWillAccept: (String? data) => true,
        onAccept: (String data) {
          setState(() {
            if (todoColumnItems.contains(data)) {
              todoColumnItems.remove(data);
            } else if (inProgressColumnItems.contains(data)) {
              inProgressColumnItems.remove(data);
            } else if (completeColumnItems.contains(data)) {
              completeColumnItems.remove(data);
            }
            inProgressColumnItems.add(data);
          });
        },
      ),
    );
  }
}
