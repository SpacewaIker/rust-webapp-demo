-- clear database
DELETE FROM song;
DELETE FROM album;
DELETE FROM album_artist_mtm;
DELETE FROM artist;

-- populate database
INSERT INTO artist (id, name, date_formed, genre)
VALUES
  (1, 'Awesome Band', '2010-01-01', 'rock'),
  (2, 'Popular Artist', '2000-01-01', 'pop');

INSERT INTO album (id, name, date_published)
VALUES
  (1, 'Lorem', '2015-01-01'),
  (2, 'Ipsum', '2001-01-01'),
  (3, 'Dolor', '2020-01-01');

INSERT INTO album_artist_mtm (id, album_id, artist_id)
VALUES
  (1, (SELECT id FROM album WHERE name='Lorem'), (SELECT id FROM artist WHERE name='Awesome Band')),
  (2, (SELECT id FROM album WHERE name='Ipsum'), (SELECT id FROM artist WHERE name='Popular Artist')),
  (3, (SELECT id FROM album WHERE name='Dolor'), (SELECT id FROM artist WHERE name='Awesome Band')),
  (4, (SELECT id FROM album WHERE name='Dolor'), (SELECT id FROM artist WHERE name='Popular Artist'));

INSERT INTO song (id, name, length_secs, album_id)
VALUES
  (1, 'The Quick', 10, 1),
  (2, 'Brown Fox', 20, 1),
  (3, 'Jumps Over', 30, 1),
  (4, 'The Lazy', 40, 1),
  (5, 'Dog', 50, 1),

  (6, 'Intro', 60, 2),
  (7, 'Best Song', 70, 2),
  (8, 'Interesting...', 80, 2),
  (9, 'Good Vibes', 90, 2),
  (10, 'Nice Ballad', 100, 2),
  (11, 'Outro', 110, 2),

  (12, 'Best Collab Song Ever', 120, 3),
  (13, 'Best Collab Song Ever (Remix)', 130, 3);
