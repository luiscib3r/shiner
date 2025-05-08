import 'dart:io';

import 'package:app/app/internal/core/images.dart';
import 'package:flutter/material.dart';
import 'package:skeletonizer/skeletonizer.dart';

class ImageCard extends StatelessWidget {
  const ImageCard({required this.image, super.key});

  final ImageJob image;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {},
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      child: Card(
        child: Column(
          spacing: 8,
          children: [
            if (image.imagePath != null)
              Image.file(File(image.imagePath!), height: 200)
            else
              ClipRRect(
                borderRadius: const BorderRadius.vertical(
                  top: Radius.circular(8),
                ),
                child: Skeletonizer(
                  child: Container(
                    height: 100,
                    width: double.infinity,
                    color: Colors.grey.shade300,
                  ),
                ),
              ),
            Padding(
              padding: const EdgeInsets.all(8),
              child: Column(
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          image.prompt,
                          overflow: TextOverflow.ellipsis,
                          maxLines: 2,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
