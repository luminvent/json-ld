use std::hash::Hash;

use json_ld_core_next::{ExpandedDocument, Indexed, Object};
use linked_data_next::{CowRdfTerm, LinkedDataResource};
use rdf_types::{
	interpretation::{
		ReverseBlankIdInterpretation, ReverseIriInterpretation, ReverseLiteralInterpretation,
	},
	vocabulary::IriVocabularyMut,
	Interpretation, Term, Vocabulary,
};

use crate::Error;

use super::{node::SerializeNode, value::literal_to_value};

pub struct SerializeDefaultGraph<'a, I, V: Vocabulary> {
	vocabulary: &'a mut V,
	interpretation: &'a mut I,
	result: &'a mut ExpandedDocument<V::Iri, V::BlankId>,
}

impl<'a, I, V: Vocabulary> SerializeDefaultGraph<'a, I, V> {
	pub fn new(
		vocabulary: &'a mut V,
		interpretation: &'a mut I,
		result: &'a mut ExpandedDocument<V::Iri, V::BlankId>,
	) -> Self {
		Self {
			vocabulary,
			interpretation,
			result,
		}
	}
}

impl<I: Interpretation, V: Vocabulary> linked_data_next::GraphVisitor<I, V>
	for SerializeDefaultGraph<'_, I, V>
where
	V: IriVocabularyMut,
	V::Iri: Clone + Eq + Hash,
	V::BlankId: Clone + Eq + Hash,
	I: ReverseIriInterpretation<Iri = V::Iri>
		+ ReverseBlankIdInterpretation<BlankId = V::BlankId>
		+ ReverseLiteralInterpretation<Literal = V::Literal>,
{
	type Ok = ();
	type Error = Error;

	fn subject<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<I, V> + linked_data_next::LinkedDataSubject<I, V>,
	{
		let id = match value
			.lexical_representation(self.vocabulary, self.interpretation)
			.map(CowRdfTerm::into_owned)
		{
			Some(Term::Literal(lit)) => {
				let value = literal_to_value(self.vocabulary, lit);
				self.result.insert(Indexed::new(Object::Value(value), None));
				return Ok(());
			}
			Some(Term::Id(id)) => Some(json_ld_core_next::Id::Valid(id)),
			_ => None,
		};

		let serializer = SerializeNode::new(self.vocabulary, self.interpretation, id);

		let node = value.visit_subject(serializer)?;
		self.result.insert(Indexed::new(Object::node(node), None));
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}
