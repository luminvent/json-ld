use linked_data_next::{
	LinkedData, LinkedDataGraph, LinkedDataPredicateObjects, LinkedDataResource, LinkedDataSubject,
	ResourceInterpretation,
};
use rdf_types::{vocabulary::IriVocabularyMut, Interpretation, Vocabulary};

use crate::{
	object::List,
	rdf::{RDF_FIRST, RDF_REST},
	IndexedObject,
};

impl<T, B, V: Vocabulary, I: Interpretation> LinkedDataResource<I, V> for List<T, B> {
	fn interpretation(
		&self,
		_vocabulary: &mut V,
		_interpretation: &mut I,
	) -> ResourceInterpretation<I, V> {
		ResourceInterpretation::Uninterpreted(None)
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedDataSubject<I, V> for List<T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit_subject<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::SubjectVisitor<I, V>,
	{
		Rest(self.as_slice()).visit_subject(visitor)
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedDataPredicateObjects<I, V>
	for List<T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::PredicateObjectsVisitor<I, V>,
	{
		visitor.object(self)?;
		visitor.end()
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedDataGraph<I, V> for List<T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit_graph<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::GraphVisitor<I, V>,
	{
		visitor.subject(self)?;
		visitor.end()
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedData<I, V> for List<T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::Visitor<I, V>,
	{
		visitor.default_graph(self)?;
		visitor.end()
	}
}

struct Rest<'a, T, B>(&'a [IndexedObject<T, B>]);

impl<T, B, V: Vocabulary, I: Interpretation> LinkedDataResource<I, V> for Rest<'_, T, B> {
	fn interpretation(
		&self,
		_vocabulary: &mut V,
		_interpretation: &mut I,
	) -> ResourceInterpretation<I, V> {
		ResourceInterpretation::Uninterpreted(None)
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedDataSubject<I, V> for Rest<'_, T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit_subject<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::SubjectVisitor<I, V>,
	{
		if let Some((first, rest)) = self.0.split_first() {
			visitor.predicate(RDF_FIRST, first.inner())?;
			visitor.predicate(RDF_REST, &Rest(rest))?;
		}

		visitor.end()
	}
}

impl<T, B, V: Vocabulary<Iri = T>, I: Interpretation> LinkedDataPredicateObjects<I, V>
	for Rest<'_, T, B>
where
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	B: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	V: IriVocabularyMut,
{
	fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: linked_data_next::PredicateObjectsVisitor<I, V>,
	{
		visitor.object(self)?;
		visitor.end()
	}
}
