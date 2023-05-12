       01 RECORD-A.
           05 STATES PIC X(2).
               88 ILLINOIS VALUE 'IL'.
               88 NEW-YORK VALUE 'NY'.
               88 MASSACHUSETTS VALUE 'MA'.
               88 INDIANA VALUE 'IN'.
           05 FLAG PIC X(2).
               88 FLAG-A VALUES 'AA' 'AB' 'AC'.
               88 FLAG-B VALUES 'BA' 'BB' 'BC'.
               88 FLAG-NUM VALUE 5 THRU 15.
           05 FIELD-WITH-VAL PIC X(2) VALUE 'BB'.
           05 SPACES-VAL PIC A(10) VALUE SPACES.
           05 ZERO-VAL PIC 9(3) VALUE ZEROS.
           05 PLUS-VAL PIC S9(3) VALUE +0.
           05 MINUS-VAL PIC S9(3) VALUE -0.
           05 DECIMAL PIC 9(2)V9(2) VALUE 12.34.