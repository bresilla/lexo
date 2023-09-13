pub mod models;
use rusqlite::{Connection, Result};
use uuid;

#[derive(Debug, Clone)]
struct BibTex {
    uuid: Option<String>, // The uuid field is used to store a unique identifier for the entry. It is not used by the standard bibliography styles, but may be used by others that produce an annotated bibliography.
    //mandatory
    author: Option<String>, // In the `author`-field, you can specify all contributors to the work you want to cite. BibTeX has several ways of specifying author names but expects in all cases a consistent and exact arrangement so that inaccurate information could lead to the unwanted output of the author's name.
    title: Option<String>, // The title field is saved for the title of a work to be cited. LaTeX-specific writing, such as capitalisation, should be taken into account.
    year: Option<String>,  // The year of publication, or creation if unpublished.
    //not mandatory
    address: Option<String>, //The address-field contains the publisher's (or any other institution) address.
    annote: Option<String>,  // BibTeX Field type
    booktitle: Option<String>, //To cite an `incollection, i.e., a part of a book/collection with its own title, or to give a title to an article, part of an inproceedings, the booktitle-field is used to distinguish. At the same time, title contains the title of the book, or incproceedings`.
    chapter: Option<String>,   // BibTeX field type: chapter
    email: Option<String>, // Email includes the email address of the stated authors if you want to indicate them. It's not a common field and might not be supported.
    edition: Option<String>, // The edition of a book, e.g. "edition = " Entering an ordinal number is recommended by default, and the first letter is usually capitalised.
    editor: Option<String>, // In the `editor`-field, you can specify all editors to the work you want to cite. BibTeX has several ways of specifying editor names but expects in all cases a consistent and exact arrangement so that inaccurate information could lead to the unwanted output of the editor's name.
    howpublished: Option<String>, // The howpublished field is used with the @misc entry, typically as a workaround for a source type BibTeX does not support. The most common example is when citing web pages and specifying a URL.
    institution: Option<String>, // The institution field is used together with the @techreport entry to specify the name of the institution that issued the report.
    journal: Option<String>,     // The name of the journal in which the journal was published.
    month: Option<String>,       // The month of publication, or creation if unpublished.
    note: Option<String>, // The notes field can be used to store additional information about a particular BibTeX entry that may be relevant to the reader or that is needed for a particular entry type and citation style but does not fit in any other field.
    number: Option<String>, // Usually the issue number (or similar identifier) of a journal, magazine or the number of technical report.
    organization: Option<String>, // Usually, an institution, the name of the conference sponsor/organizer is used here.
    pages: Option<String>, // Page range of, for example, a book. You can separate these either with commas or double dashes.
    publisher: Option<String>, // The name of the publisher of the book being cited.
    school: Option<String>, // school is specified when a phdthesis or mastersthesis is cited and is the educational institution where the student wrote the dissertation. This field is required in both entry types.
    series: Option<String>, // The field can, for example, optionally be used to specify the book series in which the book (or similar) was published.
    typee: Option<String>, // Within the BibTeX entry, the type field is used to declare an explicit type, i.e. for technical report "Research Note", or "Government Report".
    volume: Option<String>, //The 'volume' field is used to enter the volume of a journal or multi-volume book.
    //links
    crossref: Option<String>, // If you have several entries referring to the same proceedings, you can use crossref to specify fields common to several entries only once by defining crossref = . Where citationkey is the citation key of the cross-referenced entry.
    doi: Option<String>, // `doi is used to store any DOI (Digital Object Identifier of the International DOI Foundation). DOI is a digital identifier that is intended to be permanent and unique. It is used, among other things, and mostly for articles from scientific journals. It can be used in article, but more rarely also in chapter, book, or inproceedings / conference`. This field is optional in all cases.
    issn: Option<String>, // The issn field is used to store the ISSN (International Standard Serial Number) of a journal. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    isbn: Option<String>, // The isbn field is used to store the ISBN (International Standard Book Number) of a book. It is used in book, but more rarely also in article, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    url: Option<String>, // The url field is used to store the URL (Uniform Resource Locator), a link to the online version of the work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    //attitional
    abstractt: Option<String>, // The abstract field is used to store the abstract of a work, which is a summary of the work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    keywords: Option<String>, // The keywords field is used to store keywords of a work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    language: Option<String>, // The language field is used to store the language of a work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
}

struct Addidions {
    file: Option<String>,
    comment: Option<String>,
    timestamp: Option<String>,
}

#[derive(Debug, Clone)]
struct Paper {
    uuid: String,
    title: String,
    bib: BibTex,
    bib_uuid: String,
    file: Option<Vec<u8>>,
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bibtex (
            id INTEGER PRIMARY KEY,
            uuid TEXT,
            author TEXT,
            title TEXT,
            year TEXT,
            address TEXT,
            annote TEXT,
            booktitle TEXT,
            chapter TEXT,
            email TEXT,
            edition TEXT,
            editor TEXT,
            howpublished TEXT,
            institution TEXT,
            journal TEXT,
            month TEXT,
            note TEXT,
            number TEXT,
            organization TEXT,
            pages TEXT,
            publisher TEXT,
            school TEXT,
            series TEXT,
            typee TEXT,
            volume TEXT,
            crossref TEXT,
            doi TEXT,
            issn TEXT,
            isbn TEXT,
            url TEXT,
            abstractt TEXT,
            keywords TEXT,
            language TEXT
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS papers (
            uuid TEXT PRIMARY KEY,
            title TEXT,
            bib_id INTEGER,
            bib_uuid TEXT,
            file BLOB,
            FOREIGN KEY (bib_id) REFERENCES bibtex(id)
        )",
        [],
    )?;
    Ok(())
}

fn insert_bibtex(conn: &Connection, bibtex: &BibTex) -> Result<i64> {
    conn.execute(
        "INSERT INTO bibtex (
            uuid,
            author,
            title,
            year,
            address,
            annote,
            booktitle,
            chapter,
            email, 
            edition,
            editor,
            howpublished,
            institution,
            journal,
            month,
            note,
            number,
            organization,
            pages,
            publisher,
            school,
            series,
            typee,
            volume,
            crossref,
            doi,
            issn,
            isbn,
            url,
            abstractt,
            keywords,
            language
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32)",
        &[
            &bibtex.uuid,
            &bibtex.author,
            &bibtex.title,
            &bibtex.year,
            &bibtex.address,
            &bibtex.annote,
            &bibtex.booktitle,
            &bibtex.chapter,
            &bibtex.email,
            &bibtex.edition,
            &bibtex.editor,
            &bibtex.howpublished,
            &bibtex.institution,
            &bibtex.journal,
            &bibtex.month,
            &bibtex.note,
            &bibtex.number,
            &bibtex.organization,
            &bibtex.pages,
            &bibtex.publisher,
            &bibtex.school,
            &bibtex.series,
            &bibtex.typee,
            &bibtex.volume,
            &bibtex.crossref,
            &bibtex.doi,
            &bibtex.issn,
            &bibtex.isbn,
            &bibtex.url,
            &bibtex.abstractt,
            &bibtex.keywords,
            &bibtex.language,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

fn insert_paper(conn: &Connection, paper: &Paper) -> Result<()> {
    let bibtex_id = insert_bibtex(conn, &paper.bib)?;
    conn.execute(
        "INSERT INTO papers (
            uuid,
            title,
            bib_id,
            bib_uuid,
            file
        )
        VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &paper.uuid,
            &paper.title,
            &bibtex_id,
            &paper.bib_uuid,
            &paper.file,
        ),
    )?;
    Ok(())
}

pub fn main() -> Result<()> {
    let conn = Connection::open("temp/test.db")?;

    create_tables(&conn)?;

    let bibtex_uuid = uuid::Uuid::new_v4().to_string();

    let bibtex = BibTex {
        uuid: Some(bibtex_uuid.clone()),
        author: Some("author".to_string()),
        title: Some("title".to_string()),
        year: Some("year".to_string()),
        address: Some("address".to_string()),
        annote: Some("annote".to_string()),
        booktitle: Some("booktitle".to_string()),
        chapter: Some("chapter".to_string()),
        email: Some("email".to_string()),
        edition: Some("edition".to_string()),
        editor: Some("editor".to_string()),
        howpublished: Some("howpublished".to_string()),
        institution: Some("institution".to_string()),
        journal: Some("journal".to_string()),
        month: Some("month".to_string()),
        note: Some("note".to_string()),
        number: Some("number".to_string()),
        organization: Some("organization".to_string()),
        pages: Some("pages".to_string()),
        publisher: Some("publisher".to_string()),
        school: Some("school".to_string()),
        series: Some("series".to_string()),
        typee: Some("typee".to_string()),
        volume: Some("volume".to_string()),
        crossref: Some("crossref".to_string()),
        doi: Some("doi".to_string()),
        issn: Some("issn".to_string()),
        isbn: Some("isbn".to_string()),
        url: Some("url".to_string()),
        abstractt: Some("abstractt".to_string()),
        keywords: Some("keywords".to_string()),
        language: Some("language".to_string()),
    };

    let id = uuid::Uuid::new_v4();

    let paper = Paper {
        uuid: id.to_string(),
        title: "title".to_string(),
        bib: bibtex,
        bib_uuid: bibtex_uuid,
        file: None,
    };

    let err = insert_paper(&conn, &paper);
    println!("{:?}", err);
    Ok(())
}
