use ast::{types::*, P};
use lexer::rich::LiteralKind;
use session::{Ident, Span, Symbol};
use thin_vec::thin_vec;

macro_rules! sym {
	($lit:literal) => {
		Symbol::intern($lit)
	};
}

#[allow(clippy::too_many_lines)]
fn _paradigm_example_ast() -> Api {
	Api {
		attrs: thin_vec![
			Attribute {
				kind: AttrKind::DocComment(sym!(
					" This Api is a test for the documentation generator of the api"
				)),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a second line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a third line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
		],
		items: thin_vec![
			P(Item {
				attrs: thin_vec![],
				kind: ItemKind::Meta(Metadata {
					fields: thin_vec![
						P(PropertyDef {
							attrs: thin_vec![],
							ident: Ident {
								symbol: sym!("name"),
								span: Span::DUMMY
							},
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(LiteralKind::Str, sym!("Wiro's API")),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: Ident {
								symbol: sym!("description"),
								span: Span::DUMMY
							},
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(
									LiteralKind::Str,
									sym!("This is the API of Wiro")
								),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: Ident {
								symbol: sym!("version"),
								span: Span::DUMMY
							},
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(LiteralKind::Str, sym!("1.0.0")),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: Ident {
								symbol: sym!("name"),
								span: Span::DUMMY
							},
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Array(thin_vec![
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									}),
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm-preprod.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									}),
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm-dev.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									})
								]),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
					],
				}),
				ident: Ident {
					symbol: sym!(""),
					span: Span::DUMMY,
				},
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P(Item {
				attrs: thin_vec![Attribute {
					kind: AttrKind::DocComment(sym!(" Imports the `builder.dapi` file")),
					style: AttrStyle::OuterOrInline,
					id: AttrId::make_id(),
					span: Span::DUMMY,
				}],
				ident: Ident {
					symbol: sym!("builder"),
					span: Span::DUMMY,
				},
				kind: ItemKind::Scope(ScopeKind::Unloaded),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P(Item {
				attrs: thin_vec![],
				ident: Ident {
					symbol: sym!("dashboard"),
					span: Span::DUMMY,
				},
				kind: ItemKind::Scope(ScopeKind::Loaded {
					inline: true,
					items: thin_vec![P(Item {
						attrs: thin_vec![],
						ident: Ident {
							symbol: sym!("dashboard"),
							span: Span::DUMMY,
						},
						kind: ItemKind::Path(Path {
							kind: PathKind::Simple(Ident {
								symbol: sym!("dashboard"),
								span: Span::DUMMY,
							}),
							items: thin_vec![Item {
								attrs: thin_vec![],
								ident: Ident {
									symbol: sym!(""),
									span: Span::DUMMY,
								},
								kind: ItemKind::Headers(Headers {
									headers: thin_vec![
										P(FieldDef {
											attrs: thin_vec![
												Attribute {
													kind: AttrKind::DocComment(sym!(" # Safety")),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a comment"
													)),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a second line of comment"
													)),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												// @description("The API Key of the User")
												Attribute {
													kind: AttrKind::Normal(NormalAttr {
														item: AttrItem {},
													}),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												// @prefix("Api-Key")
												Attribute {
													kind: AttrKind::Normal(NormalAttr {
														item: AttrItem {},
													}),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
											],
											ident: Ident {
												symbol: sym!("Authorization"),
												span: Span::DUMMY,
											},
											ty: P(Type("".into())),
											id: NodeId::DUMMY,
											span: Span::DUMMY,
										}),
										P(FieldDef {
											// @description("The Model of the User")
											attrs: thin_vec![Attribute {
												kind: AttrKind::Normal(NormalAttr {
													item: AttrItem {},
												}),
												style: AttrStyle::OuterOrInline,
												id: AttrId::make_id(),
												span: Span::DUMMY,
											}],
											ident: Ident {
												symbol: sym!("X-Model"),
												span: Span::DUMMY,
											},
											ty: P(Type("".into())),
											id: NodeId::DUMMY,
											span: Span::DUMMY,
										}),
									],
								}),
								id: NodeId::DUMMY,
								span: Span::DUMMY,
							}],
						}),
						id: NodeId::DUMMY,
						span: Span::DUMMY,
					})],
					span: Span::DUMMY,
				}),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
		],
		id: NodeId::ROOT,
		span: Span::DUMMY,
	}
}
