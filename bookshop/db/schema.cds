using {
  Currency,
  managed,
  sap
} from '@sap/cds/common';

namespace sap.capire.bookshop;

entity Books : managed {
  key ID       : Integer;
      title    : localized String(111);
      descr    : localized String(1111);
      author   : Association to Authors;
      genre    : Association to Genres;
      stock    : Integer;
      price    : Decimal;
      currency : Currency;
      image    : LargeBinary @Core.MediaType: 'image/png';
}

entity Authors : managed {
  key ID           : Integer;
      name         : String(111);
      dateOfBirth  : Date;
      dateOfDeath  : Date;
      placeOfBirth : String;
      placeOfDeath : String;
      books        : Association to many Books
                       on books.author = $self;
}

/**
 * Hierarchically organized Code List for Genres
 */
entity Genres : sap.common.CodeList {
      @sap.hierarchy.node.for       : 'ID'
  key ID       : Integer;

      @sap.hierarchy.parent.node.for: 'ID'
      parent_ID   : Integer;

      children : Composition of many Genres
                   on children.parent_ID = $self.ID;
}

entity GenresView as
  select from Genres {
    ID,
    name,
    parent_ID as parent,
    @sap.hierarchy.level.for                : 'ID'
    (CASE WHEN parent_ID IS NULL THEN 0 ELSE 1 END)     as level:Integer,
    @sap.hierarchy.drill.state.for          : 'ID'
    (CASE WHEN $self.child_count>0 THEN 'collapsed' ELSE 'leaf' END) as expand:String,
    @sap.hierarchy.node.descendant.count.for: 'ID'
    (SELECT COUNT(1) as count FROM Genres as C WHERE C.parent_ID=Genres.ID) as child_count:Integer
  };
